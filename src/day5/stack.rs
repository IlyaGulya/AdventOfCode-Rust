use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Stack {
    pub items: Vec<char>,
}

pub fn parse_stacks(source: &str) -> Vec<Stack> {
    let mut stacks_rows_raw: Vec<&str> =
        source
            .lines()
            .take_while(|line| !line.is_empty())
            .collect();

    stacks_rows_raw.pop();

    let stacks_rows =
        stacks_rows_raw
            .into_iter()
            .map(|line| {
                let mut iter = line.chars().into_iter().peekable();

                let mut items = vec![];

                while let Ok(chunk) = iter.next_chunk::<3>() {
                    let chunk_str: String = chunk.into_iter().collect();
                    if chunk_str.trim().is_empty() {
                        items.push(None)
                    } else {
                        items.push(Some(chunk[1]))
                    }
                    iter.next();
                }

                items
            })
            .collect_vec();

    let array = array2d::Array2D::from_rows(&stacks_rows).unwrap();

    array
        .columns_iter()
        .map(|col| {
            let mut vec =
                col.collect_vec();
            vec.reverse();
            vec.into_iter()
                .flat_map(|item| {
                    if let Some(item) = item {
                        vec![item.to_owned()]
                    } else {
                        vec![]
                    }
                })
                .collect_vec()
        })
        .map(|col| Stack { items: col })
        .collect_vec()
}
