#[macro_export]
macro_rules! xml_nodes_pascal_case {
    ($($i:item)*) => { $(
        #[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq, ::serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        $i
    )* }
}

#[macro_export]
macro_rules! xml_nodes_camel_case {
    ($($i:item)*) => { $(
        #[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq, ::serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        $i
    )* }
}

#[macro_export]
macro_rules! overview_json {
    ($($i:item)*) => { $(
        #[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq, ::serde::Deserialize, ::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        $i
    )* }
}
