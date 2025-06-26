use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fmt, fs};

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format : OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers()?.clone();

    for result in reader.records() {
        //serde自动转为结构体
        let record = result?;
        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() 方法将两个迭代器组合成一个迭代器，返回一个元组
        // collect() 方法将迭代器转换为一个 JSON Value
        
        let json_value = header.iter().zip(record.iter()).collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content =  match format {
        OutputFormat::Json => {
            // 如果格式是 JSON，则不需要转换
            serde_json::to_string_pretty(&ret)?
        }
        OutputFormat::Yaml => {
            // 如果格式是 YAML，则需要转换
            serde_yaml::to_string(&ret)?
        }
    };

    //写入到文件
    fs::write(output, content)?; // => ()
    Ok(())
}
