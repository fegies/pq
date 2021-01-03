use protobuf::descriptor::FileDescriptorSet;
use serde::Serializer;
use serde_protobuf::de::DeserializerBuilder;
use serde_protobuf::descriptor::Descriptors;

pub struct PqDecoderBuilder {
    descriptors: Descriptors,
}

pub struct PqDecoder<'a> {
    deserializer_builder: DeserializerBuilder<'a>,
}

impl PqDecoderBuilder {
    pub fn new(loaded_descs: Vec<FileDescriptorSet>) -> Self {
        let mut descriptors = Descriptors::new();
        for fdset in loaded_descs {
            descriptors.add_file_set_proto(&fdset);
        }
        descriptors.resolve_refs();
        Self { descriptors }
    }

    pub fn resolve_type<'a>(&'a self, message_type: &str) -> PqDecoder<'a> {
        PqDecoder {
            deserializer_builder: DeserializerBuilder::for_named_message(
                &self.descriptors,
                message_type,
            )
            .expect("The provided message type was invalid"),
        }
    }
}

impl<'a> PqDecoder<'a> {
    pub fn transcode_message<'b, S: Serializer>(&mut self, data: &[u8], out: S) {
        let deserializer = self.deserializer_builder.for_input(data);
        serde_transcode::transcode(deserializer, out).unwrap();
    }
}
