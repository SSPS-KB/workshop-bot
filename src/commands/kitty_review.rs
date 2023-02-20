use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;
use i18n::t;
use rand::prelude::*;

pub(crate) fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("kitty")
        .description(t!("commands.kitty.description").to_string())
        .description_localized("cs", t!("commands.kitty.description", "cs"))
        .dm_permission(true)
}

fn get_message(locale: String) -> &'static str {
    match locale.as_str() {
        "cs" => t!("commands.kitty.message", "cs"),
        _ => t!("commands.kitty.message"),
    }
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let locale = command.clone().locale;

    let mut rng = rand::thread_rng();
    let my_strings : Vec<&str> = vec![
        "https://media.tenor.com/B-yfODI1VrYAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/oypPy4ae_OYAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/_j23QV67yroAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/BjHfBGElvI4AAAAM/kitty-review.gif",
        "https://media.tenor.com/Ugz5n_Xk-ZAAAAAM/kitty-review.gif",
        "https://media.tenor.com/4Xh2qykHi70AAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/5gC8yY3gy38AAAAM/kitty-review-gato.gif",
        "https://media.tenor.com/J7Si7OINgNIAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/2-dhm_j3BIgAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/RaiVsIUibZgAAAAM/kitty-review-ugly.gif",
        "https://media.tenor.com/RtHoxZDfww8AAAAM/kitty-review.gif",
        "https://media.tenor.com/ZrM6NS327DQAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/gWyFLowFTW8AAAAM/kitty-review.gif",
        "https://media.tenor.com/bL2_627lEdcAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/fcnZ4XYicmIAAAAM/kitty.gif",
        "https://media.tenor.com/yl6UCvhx1u4AAAAM/seals-emporium-kitty-review.gif",
        "https://media.tenor.com/f-MfUeJVatAAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/Xkcc5oPvHrYAAAAM/kitty-review.gif",
        "https://media.tenor.com/w42dhlxKq7kAAAAM/kitty-review.gif",
        "https://media.tenor.com/nD7y00FxgNIAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/QfBZojnprFkAAAAM/drive-kity-review.gif",
        "https://media.tenor.com/SiD4vduhteoAAAAM/kitty-review-cat-review.gif"
        "https://media.tenor.com/nZOCXqQBhGcAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/oE3BGDgwH3UAAAAM/kitty-review.gif",
        "https://media.tenor.com/UGIf7Cv-B_cAAAAM/kitten-review.gif",
        "https://media.tenor.com/36yc8B3i4z0AAAAM/kitty-review-cat-review.gif"
        "https://media.tenor.com/U-zdn0WKfXAAAAAM/plus-kitty-review.gif",
        "https://media.tenor.com/MmZxIILWyL4AAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/lfdNIF_8KSsAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/OKujvZfBgaEAAAAM/cat-review-kitty-review.gif",
        "https://media.tenor.com/PCbdiOSrbsoAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/SdNc8vOfIFQAAAAM/kitty-review-cat-review.gif"
        "https://media.tenor.com/l870LciTj48AAAAM/kitty-review.gif"
        "https://media.tenor.com/tAz6JvfAIogAAAAM/kitty-review-flipping-off.gif",
        "https://media.tenor.com/hgOBDpCCtroAAAAM/kitty-review-cute-kitty.gif",
        "https://media.tenor.com/nrm8eUGu4uoAAAAM/kitty-review.gif",
        "https://media.tenor.com/x_HH09EGTxwAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/uN6b1JmgizgAAAAM/sus-amongus.gif",
        "https://media.tenor.com/RDjGvZQo5GQAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/uNCatVzoVtQAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/brDdMo61ghMAAAAM/kitty-review.gif",
        "https://media.tenor.com/MQ9CVQKvNTwAAAAM/kitty-review-pvzh.gif",
        "https://media.tenor.com/erVgRgU9uPEAAAAM/cat-kitty-review.gif",
        "https://media.tenor.com/RuOBPHa4PZkAAAAM/kitty-review-bag.gif",
        "https://media.tenor.com/0aND1KtFBMUAAAAM/kitty-review.gif",
        "https://media.tenor.com/qoLNHHh64SgAAAAM/cat-music.gif",
        "https://media.tenor.com/ECAwQcWmgO4AAAAM/kitty-review.gif",
        "https://media.tenor.com/3oafhOTNGwgAAAAM/cat-review-kitty-review.gif",
        "https://media.tenor.com/qucrYqQR7f4AAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/1VrixSwUPPcAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/3-NQ3d5TBwQAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/MoEEiTxtiQgAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/_4QLIkvX9vwAAAAM/cat-review-kitty-review.gif",;
        "https://media.tenor.com/xw4N7-6NZsQAAAAM/kitty-review.gif",
        "https://media.tenor.com/ojQgZ9ep9qoAAAAM/cat-kitty.gif",
        "https://media.tenor.com/0CdmgvpagWwAAAAM/cat-kitty.gif",
        "https://media.tenor.com/DAZBQzzxRpoAAAAM/keqinf-keqing.gif",
        "https://media.tenor.com/5x1Eap_a9KgAAAAM/kittyreview-kitty-review-healthy-kitty.gif",
        "https://media.tenor.com/vkCslHgzA0AAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/CFJQEGK1aiUAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/DJ0XaLiVyigAAAAM/kitty-review.gif",
        "https://media.tenor.com/fDhkHpG0JksAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/yhllfW6T_SsAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/oulv0W5JfDEAAAAM/kitty-review.gif",
        "https://media.tenor.com/NZjx4Tp-cvgAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/Z2vFIZXbaR8AAAAM/kitty-review.gif",
        "https://media.tenor.com/NK3vGCC_MrEAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/KUr9Q4B5wY4AAAAM/kitty-review.gif",
        "https://media.tenor.com/EtrXzDCVTuQAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/QYGqkL-QcgcAAAAM/kitty-review-isnt-ballin.gif",
        "https://media.tenor.com/eFyErcP-k-sAAAAM/kitty-cat.gif",
        "https://media.tenor.com/sbAVRvVlsdkAAAAM/kitty-review-duck.gif",
        "https://media.tenor.com/2a3MJnMifRYAAAAM/bingus-kitty-review.gif",
        "https://media.tenor.com/-p_PxRksdYQAAAAM/kitty-review.gif",
        "https://media.tenor.com/Kz5uedewAOYAAAAM/cat-kitty.gif",
        "https://media.tenor.com/gR-J69m6XvcAAAAM/kitty-review-sunglasses.gif",
        "https://media.tenor.com/RgAG9-gQJjMAAAAM/kitty-review.gif",
        "https://media.tenor.com/8ZbSOhd7SpcAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/Bd0EqT3KApgAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/RgAG9-gQJjMAAAAM/kitty-review.gif",
        "https://media.tenor.com/9z9SCCZ3E_YAAAAM/kitty-cat.gif",
        "https://media.tenor.com/KrpqyGUzTxIAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/Kz5uedewAOYAAAAM/cat-kitty.gif",
        "https://media.tenor.com/1UaG9IQWnrwAAAAM/cat-review-kitty-review.gif",
        "https://media.tenor.com/ky2vo-DD7icAAAAM/kitty-review.gif",
        "https://media.tenor.com/3vxQehzvXSIAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/tRQYu7hjHZIAAAAM/kitty-review.gif",
        "https://media.tenor.com/varISoj5I7YAAAAM/kitty-review.gif",
        "https://media.tenor.com/y1u57bBQtxYAAAAM/floppa-big.gif",
        "https://media.tenor.com/bmId6TT8HGsAAAAM/kitty-review-galaxxyss.gif",
        "https://media.tenor.com/_XpxtM_BOiEAAAAM/kitty-review.gif",
        "https://media.tenor.com/ZVu_xistoQEAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/cLz-BZ63CNAAAAAM/kitty-review-puppy-review.gif",
        "https://media.tenor.com/nWTkt11-R5kAAAAM/kitty-kitty-review.gif",
        "https://media.tenor.com/Wq-Cy6Su0lMAAAAM/vtuber-fubuki.gif",
        "https://media.tenor.com/84yznS-rFlQAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/_bWP_bTRKSoAAAAM/kitty-review-kitty-ballin.gif",
        "https://media.tenor.com/bUplpZD7i1oAAAAM/kitty-review-smart-cat.gif",
        "https://media.tenor.com/eydZyU5UfdsAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/IUnpxMCN2bEAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/egr9B2ji6ncAAAAM/kitty-review.gif",
        "https://media.tenor.com/IUnpxMCN2bEAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/td8DMw0STB4AAAAM/cat-review-kitty-review.gif",
        "https://media.tenor.com/3ybaif9qGV4AAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/pb_Fg9KHdbEAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/29LhPNMoYfIAAAAM/kitty-review.gif",
        "https://media.tenor.com/w9mX_s0D4tUAAAAM/kitty-review.gif",
        "https://media.tenor.com/CpCES-nu_P8AAAAM/emote-legacy.gif",
        "https://media.tenor.com/ROZrj_G34QsAAAAM/cat-kate.gif",
        "https://media.tenor.com/kWquhUziNkYAAAAM/kitty-review-box.gif",
        "https://media.tenor.com/3CNMRsag9QQAAAAM/elite-dangerous-review-bomb.gif",
        "https://media.tenor.com/LCvsDxI2FQsAAAAM/cat-love.gif",
        "https://media.tenor.com/8DTzZnh9a3sAAAAM/please-help-me-cats.gif",
        "https://media.tenor.com/A7f16MoH_x4AAAAM/kitty-review-ayo.gif",
        "https://media.tenor.com/Fvmuknqyn-QAAAAM/kitty-review-houston.gif",
        "https://media.tenor.com/dr_7o_FiSUYAAAAM/kyle-demo-review.gif",
        "https://media.tenor.com/OzAGVcaeLIoAAAAM/linux-meme.gif",
        "https://media.tenor.com/TXwxEBfDNqMAAAAM/kitty-review-tim.gif",
        "https://media.tenor.com/zBfDc3TxHt4AAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/4RE6RfaDX-gAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/sBOaIwB_EQgAAAAM/saturnius-kitty.gif",
        "https://media.tenor.com/_KZCGAjNRQUAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/Z8BsAXIOSfUAAAAM/puppy-review-dog-review.gif",
        "https://media.tenor.com/PFqmrAS7tTUAAAAM/cat.gif",
        "https://media.tenor.com/5yA3Wh1M8foAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/DGhnlcL9-_kAAAAM/kity-review-corn.gif",
        "https://media.tenor.com/qrAOGtMPvJcAAAAM/kitty-review-mr-kitty.gif",
        "https://media.tenor.com/22QBSFHdObMAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/sJM8kBNhU6gAAAAM/kitty-review.gif",
        "https://media.tenor.com/WgjUlE9JkrYAAAAM/cat-review.gif",
        "https://media.tenor.com/oKrZintn9RsAAAAM/kitty-review-kitty.gif",
        "https://media.tenor.com/54VSUx13racAAAAM/kitty-review-weird-cat.gif",
        "https://media.tenor.com/D0aR0t4e5YcAAAAM/kitty-review-touch.gif",
        "https://media.tenor.com/tyyhBCSc45EAAAAM/cat-kitty.gif",
        "https://media.tenor.com/eImJP8ds7SkAAAAM/meh-leeloo.gif",
        "https://media.tenor.com/eBZQidfJapkAAAAM/cat.gif",
        "https://media.tenor.com/R-XETPPBTusAAAAM/kitty-review.gif",
        "https://media.tenor.com/NLOu5kzqjYkAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/fqJnfmqGx2MAAAAM/kitty-review-cute.gif",
        "https://media.tenor.com/wWhV2BDd178AAAAM/cat-sully.gif",
        "https://media.tenor.com/NGBk8ttHepkAAAAM/kitty-review-big-flopa.gif",
        "https://media.tenor.com/nfH77F0VJVsAAAAM/cat-kate.gif",
        "https://media.tenor.com/Pd37tBcMXl0AAAAM/bingus-kitty-review.gif",
        "https://media.tenor.com/zetthEgclgsAAAAM/hoppe-hoppersk.gif",
        "https://media.tenor.com/uDBnyg6PIbAAAAAM/kitty-review-boy-cat.gif",
        "https://media.tenor.com/MEqESMcev1oAAAAM/kitty-review.gif",
        "https://media.tenor.com/U-JzBeROAN8AAAAM/fat-cat.gif",
        "https://media.tenor.com/44p1oE84V3IAAAAM/kitty-review-cat-review.gif",
        "https://media.tenor.com/w5sWifOs9qQAAAAM/kitty-review.gif",
        "https://media.tenor.com/8VrR7Ycy5acAAAAM/digley-scuff.gif",
        "https://media.tenor.com/3Lxc0HIVs4MAAAAM/kitty-cat.gif",
        "https://media.tenor.com/pg-YMA_MBSgAAAAM/kitty-review-cat.gif",
        "https://media.tenor.com/LCV_OZdX5jIAAAAM/kitty-review-amogus.gif",
        "https://media.tenor.com/gD6gaqm6GaYAAAAM/lizard-lizard-review.gif",
        "https://media.tenor.com/n8Klta656rUAAAAM/lodorf-melorin.gif",
        "https://media.tenor.com/dYOwLAoiz2AAAAAM/cat-cate.gif",
        "https://media.tenor.com/AXI-olJtt6cAAAAM/kitty-review-cocainer.gif",
        "https://media.tenor.com/UVez5WN3xBAAAAAM/bingus-cats.gif",
        "https://media.tenor.com/zCdNE6v_GKMAAAAM/kitty-review-seal.gif",
        "https://media.tenor.com/Pj8H-4ee_MMAAAAM/cat-flexible.gif",
        "https://media.tenor.com/cJkWMVSwAy4AAAAM/flavortown-monkey-review.gif",
        "https://media.tenor.com/v-GggtEUBjsAAAAM/kitty-review-cowboy-bebop.gif",
        "https://media.tenor.com/qpNAAIF7pHEAAAAM/kapi-kittyreview.gif",
        "https://media.tenor.com/1P4gdfAXiXoAAAAM/cat.gif",
        "https://media.tenor.com/njLU1utI8aIAAAAM/flavortown-monkey-review.gif",
        "https://media.tenor.com/FreMmPqrvfgAAAAM/cat.gif",
        "https://media.tenor.com/xb4fVIuelFsAAAAM/camerupt-camerupt-review.gif",
        "https://media.tenor.com/KyVImKcVC5oAAAAM/flavortown-monkey-review.gif"
    ];
    let random_string_index: usize = rng.gen_range(0..my_strings.len());
    let string = my_strings[random_string_index];

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(get_message(locale))
                        .embed(|embed| embed
                            .image(string)
                            .color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!(
            "There was an error while responding to skull command: {}",
            e
        )
    };
}