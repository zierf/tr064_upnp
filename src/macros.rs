#[macro_export]
macro_rules! xml_nodes_pascal_case {
    ($($i:item)*) => { $(
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        $i
    )* }
}

#[macro_export]
macro_rules! xml_nodes_camel_case {
    ($($i:item)*) => { $(
        #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        $i
    )* }
}

#[macro_export]
macro_rules! overview_json {
    ($($i:item)*) => { $(
        #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        $i
    )* }
}
