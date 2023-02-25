#[derive(Debug, PartialEq, Eq)]
pub(super) struct PageNum {
    curr: u32,
    total: u32,
}

impl PageNum {
    pub(super) fn from_tag(
        tag: &tl::HTMLTag,
        parser: &tl::Parser,
    ) -> Result<PageNum, Box<dyn std::error::Error>> {
        let node_total_pn = tag
            .query_selector(parser, "span.tbui_total_page")
            .ok_or("no total_pn node")?
            .next()
            .unwrap()
            .get(parser)
            .unwrap();
        let total_pn = node_total_pn.inner_text(parser).to_string();
        let fmt_err = "invalid format";
        let total_pn = total_pn
            .strip_prefix("共")
            .ok_or(fmt_err)?
            .strip_suffix("页")
            .ok_or(fmt_err)?;
        let total: u32 = total_pn.parse().expect("invaild int");

        let node_curr_pn = tag
            .query_selector(parser, "li.active")
            .ok_or("no curr_pn node")?
            .next()
            .unwrap()
            .get(parser)
            .unwrap();
        let curr_pn = node_curr_pn.inner_text(parser).to_string();
        let curr: u32 = curr_pn.parse().expect("invaild int");

        let page_idx = PageNum { curr, total };
        return Ok(page_idx);
    }

    pub fn has_more(&self) -> bool {
        self.curr < self.total
    }

    pub fn has_prev(&self) -> bool {
        self.curr > self.total
    }
}

#[cfg(test)]
mod tests {
    use super::PageNum;
    use std::error::Error;

    #[test]
    fn test_page_num() -> Result<(), Box<dyn Error>> {
        let raw_str = r###"<div class="tbui_pagination tbui_pagination_right">
        <ul>
            <li><a class="prev_page">&lt;</a></li>
            <li><a>1</a></li>
            <li class="active"><span>2</span></li>
            <li><a>3</a></li>
            <li><a>4</a></li>
            <li><a>5</a></li>
            <li><a>6</a></li>
            <li><a class="next_page">&gt;</a></li>
        </ul><span class="tbui_total_page">共22910页</span><input class="ui_textfield pagination_input" type="text" name="pn">
    </div>"###;
        let dom = tl::parse(raw_str, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();

        let tag_page = dom
            .query_selector("div.tbui_pagination")
            .ok_or("no page node")?
            .next()
            .unwrap()
            .get(parser)
            .unwrap()
            .as_tag()
            .unwrap();

        let page_idx = PageNum::from_tag(tag_page, parser)?;

        assert_eq!(
            page_idx,
            PageNum {
                curr: 2,
                total: 22910
            }
        );

        Ok(())
    }
}
