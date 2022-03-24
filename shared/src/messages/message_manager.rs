use std::collections::HashMap;

use naia_serde::{BitReader, BitWriter};

use crate::{
    connection::packet_notifiable::PacketNotifiable,
    protocol::{
        entity_property::NetEntityHandleConverter, manifest::Manifest, protocolize::Protocolize,
    },
    types::{MessageId, PacketIndex},
    write_list_header,
};

use super::{
    channel_config::{ChannelConfig, ChannelIndex, ChannelMode},
    message_channel::MessageChannel,
    ordered_reliable_channel::OrderedReliableChannel,
    unordered_reliable_channel::UnorderedReliableChannel,
    unordered_unreliable_channel::UnorderedUnreliableChannel,
};

/// Handles incoming/outgoing messages, tracks the delivery status of Messages
/// so that guaranteed Messages can be re-transmitted to the remote host
pub struct MessageManager<P: Protocolize, C: ChannelIndex> {
    channels: HashMap<C, Box<dyn MessageChannel<P, C>>>,
    packet_to_message_map: HashMap<PacketIndex, (C, MessageId)>,
}

impl<P: Protocolize, C: ChannelIndex> MessageManager<P, C> {
    /// Creates a new MessageManager
    pub fn new(channel_config: &ChannelConfig<C>) -> Self {
        // initialize all reliable channels
        let mut channels = HashMap::new();
        let all_channel_settings = channel_config.all_channels();
        for (channel_index, channel) in all_channel_settings {
            let new_channel: Option<Box<dyn MessageChannel<P, C>>> = match channel.mode {
                ChannelMode::UnorderedUnreliable => Some(Box::new(
                    UnorderedUnreliableChannel::new(channel_index.clone()),
                )),
                ChannelMode::UnorderedReliable(settings) => Some(Box::new(
                    UnorderedReliableChannel::new(channel_index.clone(), &settings),
                )),
                ChannelMode::OrderedReliable(settings) => Some(Box::new(
                    OrderedReliableChannel::new(channel_index.clone(), &settings),
                )),
                _ => None,
            };

            if new_channel.is_some() {
                channels.insert(channel_index.clone(), new_channel.unwrap());
            }
        }

        MessageManager {
            channels,
            packet_to_message_map: HashMap::new(),
        }
    }

    pub fn collect_incoming_messages(&mut self) -> Vec<(C, P)> {
        let mut output: Vec<(C, P)> = Vec::new();
        for (_, channel) in &mut self.channels {
            channel.collect_incoming_messages(&mut output);
        }
        output
    }

    pub fn collect_outgoing_messages(&mut self, rtt_millis: &f32) {
        for (_, channel) in &mut self.channels {
            channel.collect_outgoing_messages(rtt_millis);
        }
    }

    // Outgoing Messages

    /// Returns whether the Manager has queued Messages that can be transmitted
    /// to the remote host
    pub fn has_outgoing_messages(&self) -> bool {
        return self
            .channels
            .iter()
            .any(|(_, channel)| channel.has_outgoing_messages());
    }

    /// Queues an Message to be transmitted to the remote host
    pub fn send_message(&mut self, channel_index: C, message: P) {
        if let Some(channel) = self.channels.get_mut(&channel_index) {
            channel.send_message(message);
        }
    }

    // MessageWriter

    pub fn write_messages(
        &mut self,
        writer: &mut BitWriter,
        packet_index: PacketIndex,
        converter: &dyn NetEntityHandleConverter,
    ) {
        for (channel_index, channel) in &mut self.channels {
            if let Some(message_ids) = channel.write_messages(converter, writer) {
                for message_id in message_ids {
                    self.packet_to_message_map
                        .insert(packet_index, (channel_index.clone(), message_id));
                }
            } else {
                write_list_header(writer, &0);
            }
        }
    }

    // MessageReader
    pub fn read_messages(
        &mut self,
        reader: &mut BitReader,
        manifest: &Manifest<P>,
        converter: &dyn NetEntityHandleConverter,
    ) {
        for (_, channel) in &mut self.channels {
            channel.read_messages(reader, manifest, converter);
        }
    }
}

impl<P: Protocolize, C: ChannelIndex> PacketNotifiable for MessageManager<P, C> {
    /// Occurs when a packet has been notified as delivered. Stops tracking the
    /// status of Messages in that packet.
    fn notify_packet_delivered(&mut self, packet_index: PacketIndex) {
        if let Some((channel_index, message_id)) = self.packet_to_message_map.get(&packet_index) {
            if let Some(channel) = self.channels.get_mut(channel_index) {
                channel.notify_message_delivered(message_id);
            }
        }
    }

    fn notify_packet_dropped(&mut self, _: PacketIndex) {}
}
