use clap::Parser;
use std::{fmt, fs, path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name="rcli", version, author, about, long_about = None )]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    ///处理CSV文件
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    ///生成随机密码
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    ///检查输入文件是否存在
    #[arg(short, long, value_parser = verify_input_file )]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,
    /// default_value: 类型不明确，传字符串，需实现FromStr特性，并解析为枚举类型
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    ///default_value_t: 默认值，类型明确，无需实现FromStr特性
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    ///密码长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    ///是否包含大写字母
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    ///是否包含小写字母
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    ///是否包含数字
    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    ///是否包含特殊符号
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

///检查输入文件是否存在
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exist")
    }
}


fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    ///::<T>：显式指定泛型类型 T。
    ///当目标类型无法通过上下文自动推断时。
    format.parse()
}

///枚举到字符串转换
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
    
}

///字符串到枚举转换
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => Err(anyhow::anyhow!("Invalid format: {}", v)),
            
        }
    }
    
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
    
}