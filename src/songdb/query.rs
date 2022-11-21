pub struct Query {
    pub title:      Option<String>,
    pub album:      Option<String>,
    pub artist:     Option<String>,
    pub genre:      Option<String>,
    pub year:       Option<i64>,
    pub track_num:  Option<i64>,
    pub duration:   Option<f64>, // in seconds
    pub path:       Option<String>,
    pub lyrics:     Option<String>,
    pub hash:       Option<String>,
}
