use fhir_parser::{XmlAttribute, XmlElement, XmlName, XmlTokenStream};
use proptest::prelude::*;

proptest! {
    #[test]
    fn xml_name_round_trip(local in "[a-zA-Z]+") {
        let name = XmlName::new(&local);
        assert_eq!(name.local(), local);
    }

    #[test]
    fn xml_attribute_round_trip(
        name in "[a-zA-Z]+",
        value in "[a-zA-Z0-9]+"
    ) {
        let attr = XmlAttribute::local(&name, &value);
        assert_eq!(attr.value(), value);
        assert_eq!(attr.name().local(), name);
    }

    #[test]
    fn xml_element_round_trip(name in "[a-zA-Z]+") {
        let elem = XmlElement::new(&name);
        assert_eq!(elem.name(), name);
        assert!(elem.children().is_empty());
    }

    #[test]
    fn xml_token_stream_preserves_order(items in prop::collection::vec("[a-z]+", 0..10)) {
        let stream = XmlTokenStream::new(items.clone());
        let output = stream.into_tokens();
        assert_eq!(items, output);
    }

    #[test]
    fn xml_token_stream_deterministic(items in prop::collection::vec("[a-z]+", 0..10)) {
        let stream = XmlTokenStream::new(items);
        let yaml1 = format!("{:?}", stream);
        let yaml2 = format!("{:?}", stream);
        assert_eq!(yaml1, yaml2);
    }
}
