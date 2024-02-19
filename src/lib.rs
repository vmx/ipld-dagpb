//! Protobuf codec.
#![deny(missing_docs)]

mod codec;
mod error;

use bytes::Bytes;
use cid::Cid;
use ipld_core::ipld::Ipld;

use crate::{
    codec::{PbNode, PbNodeRef},
    error::Error,
};

/// Convert from [`ipld_core::ipld::Ipld`] into serialized DAG-PB.
pub fn from_ipld(ipld: &Ipld) -> Result<Vec<u8>, Error> {
    let node: PbNodeRef = ipld.try_into()?;
    Ok(node.into_bytes())
}

/// Convert from serialized DAG-PB to [`ipld_core::ipld::Ipld`].
pub fn to_ipld(bytes: &[u8]) -> Result<Ipld, Error> {
    let node = PbNode::from_bytes(Bytes::copy_from_slice(bytes))?;
    Ok(node.into())
}

/// Extract all the links from a serialize DAG-PB object.
pub fn links(bytes: &[u8], links: &mut impl Extend<Cid>) -> Result<(), Error> {
    let node = PbNode::from_bytes(Bytes::copy_from_slice(bytes))?;
    for link in node.links {
        links.extend(Some(link.cid));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    #[test]
    fn test_encode_decode() {
        let cid =
            Cid::try_from("bafkreie74tgmnxqwojhtumgh5dzfj46gi4mynlfr7dmm7duwzyvnpw7h7m").unwrap();
        let mut pb_link = BTreeMap::<String, Ipld>::new();
        pb_link.insert("Hash".to_string(), cid.into());
        pb_link.insert("Name".to_string(), "block".to_string().into());
        pb_link.insert("Tsize".to_string(), 13.into());

        let links: Vec<Ipld> = vec![pb_link.into()];
        let mut pb_node = BTreeMap::<String, Ipld>::new();
        pb_node.insert("Data".to_string(), b"Here is some data\n".to_vec().into());
        pb_node.insert("Links".to_string(), links.into());
        let data: Ipld = pb_node.into();

        let bytes = from_ipld(&data).unwrap();
        let data2 = to_ipld(&bytes).unwrap();
        assert_eq!(data, data2);
    }
}
