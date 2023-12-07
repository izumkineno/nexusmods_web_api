use std::fmt::Display;
use serde::{Deserialize, Serialize};

/// 排序：正反
/// 默认：反
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Order {
    /// 正序
    ASC,
    /// 倒序
    DESC,
    /// 无
    None,
}
impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Order::ASC => ",order:ASC",
            Order::DESC => ",order:DESC",
            Order::None => "",
        };
        write!(f, "{}", str)
    }
}

/// 排序：方式
/// 默认：发布时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoftBy {
    /// 发布时间
    DatePublished,
    /// 赞的数量
    Endorsements,
    /// 下载量
    Downloads,
    /// 唯一下载量（单用户多次算一次）
    UniqueDownloads,
    /// 最后更新
    LastUpdated,
    /// 作者
    AuthorName,
    /// 文件名
    FileName,
    /// 文件大小
    FileSize,
    /// 最近评论
    Trending,
    /// 热门趋势
    LastComment,
    /// 随机
    Random,
    /// 无
    None,
}
impl Display for SoftBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SoftBy::DatePublished => ",sort_by:date",
            SoftBy::Endorsements => ",sort_by:OLD_endorsements",
            SoftBy::Downloads => ",sort_by:OLD_downloads",
            SoftBy::UniqueDownloads => ",sort_by:OLD_u_downloads",
            SoftBy::LastUpdated => ",sort_by:lastupdate",
            SoftBy::AuthorName => ",sort_by:author",
            SoftBy::FileName => ",sort_by:name",
            SoftBy::FileSize => ",sort_by:OLD_size",
            SoftBy::Trending => ",sort_by:two_weeks_ratings",
            SoftBy::LastComment => ",sort_by:lastcomment",
            SoftBy::Random => ",sort_by:RAND()",
            SoftBy::None => "",
        };
        write!(f, "{}", str)
    }
}

/// 筛选：发布时间
/// 默认：所有
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimePublish {
    All,
    OneDay,
    OneWeek,
    TwoWeek,
    OneMonth,
    OneYear,
    Range,
    None,
}
impl Display for TimePublish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TimePublish::None => "",
            TimePublish::All => ",time:0",
            TimePublish::OneDay => ",time:1",
            TimePublish::OneWeek => ",time:7",
            TimePublish::TwoWeek => ",time:14",
            TimePublish::OneMonth => ",time:30",
            TimePublish::OneYear => ",time:365",
            TimePublish::Range => ",time:-1",
        };
        write!(f, "{}", str)
    }
}


/// game 搜索筛选
/// 默认；mod数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSoftBy {
    ModCount,
    Name,
    DateAdded,
    DownloadCount,
}
impl Display for GameSoftBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GameSoftBy::ModCount => "mods",
            GameSoftBy::Name => "name",
            GameSoftBy::DateAdded => "approved_date",
            GameSoftBy::DownloadCount => "downloads",
        };
        write!(f, "{}", str)
    }
}


/// mod详细界面类型
/// 默认：description
#[derive(Debug)]
pub enum ModInfoType {
    Description,
    Files,
    Images,
    // Videos,
    // Posts,
    // Bugs,
}
impl Display for ModInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ModInfoType::Description => "ModDescriptionTab",
            ModInfoType::Files => "ModFilesTab",
            ModInfoType::Images => "ModImagesTab",
            // ModInfoType::Videos => "ModVideosTab",
            // ModInfoType::Posts => "CommentContainer",
            // ModInfoType::Bugs => "ModBugsTab",
        };
        write!(f, "{}", str)
    }

}