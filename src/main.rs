use rss::Channel;
use std::error::Error;
use whatlang::detect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let links = vec![
        "https://enews.hamariweb.com/feed/",
        "https://www.dawn.com/feeds/home",
        "https://www.urdupoint.com/en/sitemap/news.rss",
        "https://www.bolnews.com/feed/",
        "https://tribune.com.pk/feed/home",
        "https://en.dailypakistan.com.pk/rss/latest",
        "https://jang.com.pk/rss/1/1",
        "https://www.express.pk/feed",
        "https://www.aaj.tv/feeds/latest-news",
        "https://www.pakistantoday.com.pk/feed/",
        "https://mashriqtv.pk/feed/",
        "https://awamiweb.com/feed",
        "https://www.jasarat.com/feed/",
        "https://chitraltimes.com/feed/",
        "https://www.thecorrespondent.pk/feed/",
        "https://aajkal.com.pk/feed/",
        "https://www.thefridaytimes.com/feed/",
        "https://thefinancialdaily.com/feed/",
        "https://www.pktribune.com/feed",
        "https://dailythepatriot.com/feed/",
        "https://www.lhrtimes.com/feed/",
        " ",
        "https://dailyqudrat.pk/feed/",
        "https://urdu.nayadaur.tv/feed/",
        " ",
        "https://www.yesurdu.com/feed",
        "https://www.bolnewsurdu.com/feed/",
        "https://urdunews.cc/feed/",
        "https://arynews.tv/feed/",
        "https://feeds.feedburner.com/com/Yeor",
        "https://www.independenturdu.com/rss.xml",
        "",
        "https://www.24urdu.com/rss/world",
        "",
        "",
        "",
        "https://pni.net.pk/feed/",
        "https://urdu.92newshd.tv/feed",
        "https://naibaat.pk/feed/",
        "https://tribune.com.pk/feed/latest",
        "https://www.samaa.tv/feeds/latest-news",
        "https://thefrontierpost.com/feed/",
        "",
        "",
        "https://www.nation.com.pk/rss/newspaper",
        "https://ummat.net/feed/",
        "https://pakobserver.net/feed",
        "https://www.brecorder.com/feeds/latest-news",
        "https://www.pakistangulfeconomist.com/feed/",
        "",
        "https://www.arabnews.pk/rss.xml",
        "https://www.samaaenglish.tv/feeds/",
        "https://92newshd.tv/feed",
        "https://thecurrent.pk/feed/",
        "https://propakistani.pk/feed",
        "",
        "https://www.inp.net.pk/inp/rss",
        "https://dailynewspk.com/feed/",
        "https://gnnhd.tv/rss/latest",
        "",
        "https://customnews.pk/feed/",
        "https://www.24newshd.tv/rss/latest",
        "https://timesofislamabad.com/rss/latest-news",
        "https://publicnews.com/feed/",
        "http://feeds.pakistannews.net/rss/8c3d7d78943a99c7",
        "",
        "",
        "",
        "https://pakistannewsexpress.com/feed/",
        "https://www.dialoguepakistan.com/feed/",
        "https://hq.com.pk/feed/",
        "https://dailykhabrain.com.pk/feed/",
        "https://thepakistan.pk/feed/",
        "https://dailymailnews.pk/feed/",
        "https://soontimes.pk/feed/",
        "https://thenamal.com/feed",
        "",
        "http://www.allpakistaninews.com/feed",
        "https://urdunews.agency/feed/",
        "",
        "https://dhartinews.com/feed/",
        "https://urdunewslab.com/?feed=rss2",
        "",
    ];

    for (i, link) in links.into_iter().enumerate() {
        match i + 1 {
            4 | 16 | 19 | 22 | 25 | 27 | 32 | 34 | 35 | 36 | 38 | 43 | 44 | 50 | 53 | 55 | 56
            | 60 | 65 | 66 | 67 | 68 | 70 | 77 | 78 | 80 | 83 => continue,
            _ => (),
        }

        let content = reqwest::get(link).await?.text().await?;

        let channel = content.parse::<Channel>().expect("parse error");

        let item = channel.items().get(0).unwrap();

        println!("Website:  {}\n", channel.title());

        println!(
            "Language: {}\n",
            detect(item.title().unwrap()).unwrap().lang().eng_name()
        );
        println!(
            "Last post: {} (link: {})\n",
            item.title().unwrap(),
            item.link().unwrap()
        );

        println!("Categories: ");
        for category in item.categories() {
            println!("{}", category.name());
        }

        println!("\n Text: {:?}\n", item.description());

        if item.author() != None {
            println!("Author: {:?}\n\n\n", item.author());
        } else if item.dublin_core_ext() != None {
            println!(
                "Author: {:?}\n\n\n",
                item.dublin_core_ext().unwrap().creators()
            );
        }
    }

    Ok(())
}
