use serde_json::json;

use crate::{
    music_aggregator::{music_aggregator_online::SearchMusicAggregator, MusicAggregator},
    music_list::MusicList,
    platform_integrator::{
        kuwo::{self, KUWO},
        utils::{kw_extract_id, wy_extract_id},
        wangyi::{self, WANGYI},
        ALL,
    },
};

use super::OnlineFactory;

impl OnlineFactory {
    pub async fn search_musiclist(
        source: &str,
        content: &str,
        page: u32,
        limit: u32,
    ) -> Result<Vec<MusicList>, anyhow::Error> {
        match source {
            ALL => {
                let kuwo_future = kuwo::search_music_list(content, page, limit);
                let wangyi_future = wangyi::search_music_list(content, page, limit);

                let (kuwo_result, wangyi_result) = tokio::join!(kuwo_future, wangyi_future);

                let mut res = Vec::new();
                if let Ok(music_lists) = kuwo_result {
                    res.extend(music_lists);
                }
                if let Ok(music_lists) = wangyi_result {
                    res.extend(music_lists);
                }
                Ok(res)
            }
            KUWO => kuwo::search_music_list(content, page, limit).await,
            WANGYI => wangyi::search_music_list(content, page, limit).await,
            _ => Err(anyhow::anyhow!("source not supported")),
        }
    }

    pub async fn get_musiclist_from_share(
        share_url: &str,
    ) -> Result<(MusicList, Vec<MusicAggregator>), anyhow::Error> {
        let limit = 30;
        if share_url.contains("music.163.com") {
            if let Some(id) = wy_extract_id(share_url) {
                let (music_list, musics) =
                    wangyi::get_musics_from_music_list(id.parse()?, 1, limit).await?;
                return Ok((
                    music_list,
                    musics
                        .into_iter()
                        .map(|m| {
                            Box::new(SearchMusicAggregator::from_music(
                                m.get_default_music().clone(),
                            )) as MusicAggregator
                        })
                        .collect(),
                ));
            }
        } else if share_url.contains("kuwo.") {
            if let Some(id) = kw_extract_id(share_url) {
                let (music_list, musics) =
                    kuwo::kuwo_music_list::get_musics_of_music_list(&id, 1, limit).await?;
                return Ok((
                    music_list,
                    musics
                        .into_iter()
                        .map(|m| Box::new(SearchMusicAggregator::from_music(m)) as MusicAggregator)
                        .collect(),
                ));
            }
        }
        return Err(anyhow::anyhow!("Failed to get musiclist and musics"));
    }
}

#[cfg(test)]
mod test {
    use crate::{factory::online_factory::OnlineFactory, platform_integrator::ALL};

    #[tokio::test]
    pub async fn test_search_musiclist() {
        let res = super::OnlineFactory::search_musiclist(&ALL, "周杰伦", 1, 10)
            .await
            .unwrap();
        res.iter().for_each(|x| {
            println!("{}", x.get_musiclist_info());
        });
        let first = res.first().unwrap();
        let musics = first.get_music_aggregators(1, 10).await.unwrap();
        musics.iter().for_each(|x| {
            println!("{}", x);
        });
    }
    #[tokio::test]
    pub async fn test_share_musiclist() {
        let urls = [
        "分享Z殘心的歌单《米津玄师》https://y.music.163.com/m/playlist?app_version=8.9.20&id=6614178314&userid=317416193&dlt=0846&creatorId=317416193 (@网易云音乐)",
        "https://m.kuwo.cn/newh5app/playlist_detail/2281251663?from=ip&t=qqfriend",
        "https://music.163.com/#/playlist?app_version=8.9.20&id=645765966&dlt=0846&creatorId=19881477",
         "分享张_成文创建的歌单「我喜欢的音乐」: https://y.music.163.com/m/playlist?id=492958277&userid=350627502&creatorId=350627502 (来自@网易云音乐)"
         ];
        for url in urls {
            let (musiclist, musics) = OnlineFactory::get_musiclist_from_share(url).await.unwrap();
            println!("{}", musiclist);
            println!("{}", musics.len());
            // musics.iter().for_each(|m| println!("{}", m))
        }
    }
}
