#[macro_export]
macro_rules! xml_nodes {
    ($($i:item)*) => { $(
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        $i
    )* }
}
