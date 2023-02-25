struct Item {
    ori: Origin,
    op: Operation,
}

struct Operation {
    un: String,
    op: u32,
    time: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Origin {
    tid: u64,
    pid: u64,
    title: String,
    text: String,
    media: Vec<String>,
    un: String,
    portrait: String,
}

impl Origin {
    pub fn from_tag(
        tag: &tl::HTMLTag,
        parser: &tl::Parser,
    ) -> Result<Origin, Box<dyn std::error::Error>> {
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

        let page_idx = Origin { curr, total };
        return Ok(page_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::Origin;
    use std::error::Error;

    #[test]
    fn test_item() -> Result<(), Box<dyn Error>> {
        let raw_str = r###"<tr>
        <td class="left_cell">
            <article class="post_wrapper clearfix">
                <div class="post_meta">
                    <div class="post_author"><a target="_blank"
                            href="/home/main?id=tb.1.bf0b322f.keWa4ldw3G4dWCNmxawCCw&amp;ie=utf-8&amp;fr=bawu">用户名: 藤本贱树</a>
                    </div>
                    <div class="post_author"><a target="_blank"
                            href="/home/main?id=tb.1.bf0b322f.keWa4ldw3G4dWCNmxawCCw&amp;ie=utf-8&amp;fr=bawu">昵称:
                            四十九新人斩</a></div><time class="ui_text_desc">02月25日 15:02</time>
                </div>
                <div class="post_content">
                    <h1><a target="_blank" href="/p/8279917875?fid=21841105&amp;pid=146953335585#146953335585"
                            title="回复：怎么评价？">回复：怎么评价？</a></h1>
                    <div class="post_text"> 没关系 我喜欢</div>
                    <div class="post_media"> </div>
                </div>
            </article>
        </td>
        <td><span class="label_12">删贴</span></td>
        <td><a href="#" class="ui_text_normal">不要揪我尾巴了</a></td>
        <td>2023-02-25<br>15:10</td>
    </tr>"###;
        let dom = tl::parse(raw_str, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();

        let tag_page = dom
            .query_selector("tr")
            .ok_or("no page node")?
            .next()
            .unwrap()
            .get(parser)
            .unwrap()
            .as_tag()
            .unwrap();

        let td_nodes = tag_page.query_selector(parser, "td").ok_or("no td node")?;

        let ori_tag = td_nodes
            .next()
            .ok_or("no ori node")?
            .get(parser)
            .unwrap()
            .as_tag()
            .unwrap();
        let ori = Origin::from_tag(ori_tag, parser)?;
        assert_eq!(
            ori,
            Origin {
                tid: 8279917875,
                pid: 146953335585,
                title: String::from("回复：怎么评价？"),
                text: String::from("没关系 我喜欢"),
                media: vec![],
                un: String::from("藤本贱树"),
                portrait: String::from("tb.1.bf0b322f.keWa4ldw3G4dWCNmxawCCw")
            }
        );

        

        Ok(())
    }
}
