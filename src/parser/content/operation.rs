struct _PageIdx {
    total: u32,
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
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

        let node_span = tag_page
            .query_selector(parser, "span.tbui_total_page")
            .ok_or("no span node")?
            .next()
            .unwrap()
            .get(parser)
            .unwrap();
        let total_str = node_span.inner_text(parser).to_string();
        assert_eq!(total_str, String::from("共22910页"));
        let fmt_err = "invalid format";
        let total_str = total_str
            .strip_prefix("共")
            .ok_or(fmt_err)?
            .strip_suffix("页")
            .ok_or(fmt_err)?;
        let total: u32 = total_str.parse().expect("invaild int");
        assert_eq!(total, 22910);

        Ok(())
    }
}
