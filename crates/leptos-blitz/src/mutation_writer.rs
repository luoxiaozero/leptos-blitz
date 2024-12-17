use blitz_dom::Document;
use leptos::prelude::Owner;
use tokio::task::LocalSet;

// #[derive(Debug)]
// pub struct LeptosState {
//     owner: Owner,
//     mountable: Box<dyn Mountable<LeptosDom>>,
//     local_set: LocalSet,
// }

// impl LeptosState {
//     pub fn create(doc: &mut Document) -> Self {
//         let root = doc.root_element();
//         let root_id = root.id;

//         Self {
//             templates: FxHashMap::default(),
//             stack: vec![root_id],
//             node_id_mapping: vec![Some(root_id)],
//         }
//     }
// }