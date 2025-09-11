// Re-export mqtt-protocol-core types
// Core protocol types and traits
pub use mqtt_protocol_core::mqtt::Version;

// Packet module with traits and types
pub mod packet {
    // Essential traits
    pub use mqtt_protocol_core::mqtt::packet::{GenericPacketTrait, IsPacketId};

    // Generic packet types
    pub use mqtt_protocol_core::mqtt::packet::{
        GenericPacket, GenericStorePacket, Packet, PacketType, Qos, StorePacket, SubEntry, SubOpts,
    };

    // Version-specific packets
    pub mod v5_0 {
        pub use mqtt_protocol_core::mqtt::packet::v5_0::*;
    }

    pub mod v3_1_1 {
        pub use mqtt_protocol_core::mqtt::packet::v3_1_1::*;
    }

    // Properties
    pub use mqtt_protocol_core::mqtt::packet::{
        AssignedClientIdentifier, AuthenticationData, AuthenticationMethod, ContentType,
        CorrelationData, MaximumPacketSize, MaximumQos, MessageExpiryInterval, PayloadFormat,
        PayloadFormatIndicator, Properties, PropertiesParse, PropertiesSize, Property, PropertyId,
        ReasonString, ReceiveMaximum, RequestProblemInformation, RequestResponseInformation,
        ResponseInformation, ResponseTopic, RetainAvailable, RetainHandling, ServerKeepAlive,
        ServerReference, SessionExpiryInterval, SharedSubscriptionAvailable,
        SubscriptionIdentifier, SubscriptionIdentifierAvailable, TopicAlias, TopicAliasMaximum,
        UserProperty, WildcardSubscriptionAvailable, WillDelayInterval,
    };
}

pub mod common {
    pub use mqtt_protocol_core::mqtt::common::{HashMap, HashSet};
}

// Role module
pub mod role {
    pub use mqtt_protocol_core::mqtt::role::*;
}

// Result code module
pub mod result_code {
    pub use mqtt_protocol_core::mqtt::result_code::*;
}

// Prelude module
pub mod prelude {
    pub use mqtt_protocol_core::mqtt::prelude::*;
}
