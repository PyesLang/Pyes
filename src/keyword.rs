use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
   "await" => Keyword::AWAIT,
   "true" => Keyword::TRUE,
   "false" => Keyword::FALSE,
   "return" => Keyword::RETURN,
};

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
   KEYWORDS.get(keyword).cloned()
}

#[derive(Debug, Clone)]
pub enum Keyword {
   ASSERT,
   AWAIT,
   TRUE,
   FALSE,
   RETURN,
}
