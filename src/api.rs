use std::collections::HashMap;
use std::sync::LazyLock;

use chrono::NaiveDate;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

pub static CLOUDFLARE_RESOURCES: LazyLock<HashMap<usize, String>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(123, "5569301a-5036-4287-720d-f1ed69d66a00".into());
    m.insert(122, "5731b95e-92d9-4e04-547e-d8bbfc857c00".into());
    m.insert(121, "1565fbd9-288d-4cd3-5aba-87bee19fb500".into());
    m.insert(120, "b867715a-f0b8-4264-defa-49d467ee9300".into());
    m.insert(119, "396818b0-7178-4097-92e8-d036f739cb00".into());
    m.insert(118, "48dadd3a-75f9-4820-9e28-392a9efcb600".into());
    m.insert(117, "832112b0-f45a-49d1-c8ee-2595412cc500".into());
    m.insert(116, "e2939cbc-4c70-434a-db7c-91029a18ad00".into());
    m.insert(115, "eee77907-12a2-4d41-9bce-219e5a59fd00".into());
    m.insert(114, "b575a7b0-a597-456a-628f-892636439800".into());
    m.insert(113, "82b40bf9-3452-4338-b968-8444a28a5300".into());
    m.insert(112, "f1e4f033-826e-4b14-9437-1a505ab4bd00".into());
    m.insert(111, "790a33f4-e7b0-455f-d1c5-cc6408753200".into());
    m.insert(110, "a30341ba-bbeb-456f-ef7f-97d6395bbd00".into());
    m.insert(109, "c77f7576-6d04-4336-ba0d-8cebb41c0b00".into());
    m.insert(108, "ebf56d58-9cbd-4dbd-ef4a-2d9b66d33e00".into());
    m.insert(107, "e2809ec9-8b5e-487e-68b7-5f138f3cf600".into());
    m.insert(106, "872dcb19-57a1-4856-55b8-b9fe5c2bbe00".into());
    m.insert(105, "c587d03b-1f38-4455-4253-3f48d77b2500".into());
    m.insert(104, "e5c8d4d8-302c-4073-72a2-a88946e37200".into());
    m.insert(103, "aef5ae14-2a18-41b6-6a96-8b838819b100".into());
    m.insert(102, "10f7e5c4-f16f-4727-1f26-4da129c17c00".into());
    m.insert(101, "396818b0-7178-4097-92e8-d036f739cb00".into());
    m.insert(100, "801c538b-e86e-4032-ce2e-139b10583a00".into());
    m.insert(99, "90a81311-63f6-4800-01b7-618e9779e700".into());
    m.insert(98, "d227a3cf-c800-4806-2301-b0907360ab00".into());
    m.insert(97, "6d544b82-a8f6-4a26-7dc2-56db925a2b00".into());
    m.insert(96, "24e42729-59f8-4336-27c9-1e27f9440d00".into());
    m.insert(95, "d16da0af-bfa8-42fe-788b-c3eb4b0a9b00".into());
    m.insert(94, "a147c87a-c360-4c53-8e3f-7a3c2a942900".into());
    m.insert(93, "9fe48554-fe08-4cb4-d5d0-b223e360ae00".into());
    m.insert(92, "ae6a83be-212c-4419-df1b-ae68d1534600".into());
    m.insert(91, "4dfe27a8-9dc5-43d0-a63a-a505fa90b800".into());
    m.insert(90, "15c0c4f6-715e-4c3f-12b9-93204725a900".into());
    m.insert(89, "1565fbd9-288d-4cd3-5aba-87bee19fb500".into());
    m.insert(88, "f0e7a947-1f5a-427f-5027-1bddac748800".into());
    m.insert(87, "52f78fa4-39c9-4d72-9531-ce8aabdefd00".into());
    m.insert(86, "b9f7fc2d-20e8-44f0-3b64-55d99ef3fa00".into());
    m.insert(85, "2e6d6b8a-0ca4-45b1-1fcf-9539308ea100".into());
    m.insert(84, "c1d5e8ac-4705-4d34-6256-bcf7d21c7500".into());
    m.insert(83, "ab30c6f5-bee7-4cec-5c6e-65c648d94700".into());
    m.insert(82, "215d814b-950b-46fb-4d5a-f9e089042700".into());
    m.insert(81, "9c823d31-4755-499f-dc2d-1440f96dcb00".into());
    m.insert(80, "9fc06d2f-92a7-41b5-2bcb-10b3f4003a00".into());
    m.insert(79, "2220fdbb-afe0-4ec8-a81c-f7cac6dbe300".into());
    m.insert(78, "eac53618-562a-41e7-95f6-25fccf663800".into());
    m.insert(77, "9ca9e2fc-37f5-48ab-5ebd-ab734ce44f00".into());
    m.insert(76, "164032ef-906c-4bac-770c-6c8a46d09200".into());
    m.insert(75, "5002639f-5e0a-4151-bed1-ad09a2beef00".into());
    m.insert(74, "adcbb7fd-3c06-4312-ae18-4008220adb00".into());
    m.insert(73, "21f1b452-0bec-480a-8da7-9e60007d0000".into());
    m.insert(72, "4855fa53-d56f-4cc9-ea99-bb36cb16bc00".into());
    m.insert(71, "de59a11d-cf13-45c1-32dc-901d89972b00".into());
    m.insert(70, "48dadd3a-75f9-4820-9e28-392a9efcb600".into());
    m.insert(69, "61fb5c57-67c5-4105-dc74-74f82fe31a00".into());
    m.insert(68, "c1d323f6-3aec-4b73-fa77-ba28fbb51f00".into());
    m.insert(67, "6620beec-26c4-49bb-9241-0b7788dda600".into());
    m.insert(66, "1602dc57-48f3-437c-7187-9733ba716300".into());
    m.insert(65, "b575a7b0-a597-456a-628f-892636439800".into());
    m.insert(64, "45f42994-eef5-491a-e2d9-c7362f87a500".into());
    m.insert(63, "2ff03529-da47-43a4-9a6d-ddbcc74c5700".into());
    m.insert(62, "2fad15e0-1b9c-4507-037b-3379e2fc4200".into());
    m.insert(61, "fa26f520-8a13-4ab7-2831-ab8763c20200".into());
    m.insert(60, "7161a837-0cca-4d03-ab6a-3fa261bc2c00".into());
    m.insert(59, "1a8bfd85-e4a2-4171-64b6-d62cf68eea00".into());
    m.insert(58, "54174a81-062a-4a0d-a6b8-7be6ffbd0b00".into());
    m.insert(57, "4b154d6c-c70f-4d0d-5cb7-da831646da00".into());
    m.insert(56, "d7e0b543-1120-4cb8-13a4-0ae217105500".into());
    m.insert(55, "c31828c9-cab1-4d46-46c7-a88c9a194d00".into());
    m.insert(54, "4fc0d8b0-2207-4a67-dcdf-ab376f4a4b00".into());
    m.insert(53, "b642ca9e-accb-43e4-e23f-6b295f9c4f00".into());
    m.insert(52, "d01acc21-1b12-4040-d967-f3b0c80fed00".into());
    m.insert(51, "2f8534c3-fbc0-42aa-90ff-5ce1d3ec0300".into());
    m.insert(50, "832112b0-f45a-49d1-c8ee-2595412cc500".into());
    m.insert(49, "97d6b0d8-0f07-447b-977a-50f2d43bdf00".into());
    m.insert(48, "b0f4aef0-7e5d-4392-5c07-c49b25dc9c00".into());
    m.insert(47, "d914c33a-0407-4453-4261-b1fadcc9f300".into());
    m.insert(46, "a5988acf-20b5-4ccc-c540-fb8cbf112500".into());
    m.insert(45, "d779a424-7555-4ef5-74b1-1f6d9ec13200".into());
    m.insert(44, "c34d2cb7-a9e8-414e-eceb-a61868f04a00".into());
    m.insert(43, "f2e8bed9-b638-49f0-6b2c-68e3009af200".into());
    m.insert(42, "9b1f89a1-b5bf-41d5-ea5d-8800b61b1600".into());
    m.insert(41, "f30a3878-2122-4c30-dfc5-c27103db2300".into());
    m.insert(40, "53eec7d8-1307-4bbd-e283-a88a182da400".into());
    m.insert(39, "9c3675de-b300-47fe-9411-dd2d193abf00".into());
    m.insert(38, "7f1abd3f-5c20-4915-f778-32a82f4aa400".into());
    m.insert(37, "e2939cbc-4c70-434a-db7c-91029a18ad00".into());
    m.insert(36, "eee77907-12a2-4d41-9bce-219e5a59fd00".into());
    m.insert(35, "fc48a0b8-75f9-473c-3db7-0409ffcb9d00".into());
    m.insert(34, "fb0f085b-ae8d-48a7-689d-d323adf10c00".into());
    m.insert(33, "6ee56395-af22-4729-18eb-f8aadc568900".into());
    m.insert(32, "897b3896-bd5f-4ddc-ffb9-825434704a00".into());
    m.insert(31, "2e40ba83-a00f-4604-c24d-b5ea54e85300".into());
    m.insert(30, "80b39267-e70a-43c5-c1f7-b25f3195a300".into());
    m.insert(29, "4af7ea4d-4317-47a7-0ff4-8ae47765a400".into());
    m.insert(28, "585ec837-3922-4a0a-b5f6-3e9ad1665700".into());
    m.insert(27, "0d9e1cb9-9e1b-4e95-e53b-7516be582d00".into());
    m.insert(26, "c0b901b3-712d-440d-93bb-97733fcfd000".into());
    m.insert(25, "9ffa50ff-c8f0-468f-07cf-08ed866b6700".into());
    m.insert(24, "03d787ce-bc69-47ca-ca1e-914a946aef00".into());
    m.insert(23, "a477693f-359c-493c-a87b-e46536235300".into());
    m.insert(22, "ab79109b-c342-4ce9-2304-bee536658700".into());
    m.insert(21, "4a197d33-2b93-424b-c057-c94e47f88b00".into());
    m.insert(20, "c935540b-836b-4539-dac1-4cfbbaf9d500".into());
    m.insert(19, "1ca8e7f5-9e4c-4521-7073-6808a1a5d700".into());
    m.insert(18, "403d54a5-853d-4972-2d0c-0a0cb3448200".into());
    m.insert(17, "5731b95e-92d9-4e04-547e-d8bbfc857c00".into());
    m.insert(16, "44668d00-2478-428d-24ee-dbea830e6900".into());
    m.insert(15, "779cdc16-5743-433a-0be6-ec51f6204600".into());
    m.insert(14, "d911d8db-c11a-4a06-442a-36b13c263f00".into());
    m.insert(13, "30ec570c-07e5-4398-6f76-57274ced6700".into());
    m.insert(12, "dfd6ca7f-51d9-4762-2e66-5d2898438600".into());
    m.insert(11, "12de5fb0-c2bb-4202-6268-dac20fae0e00".into());
    m.insert(10, "d227a3cf-c800-4806-2301-b0907360ab00".into());
    m.insert(9, "8ff6f5cf-e9fe-4418-d6c9-105fe6030000".into());
    m.insert(8, "741f4f7b-225f-4e4d-5c2a-4d926162e300".into());
    m.insert(7, "9876894a-5c5a-4cbb-a219-6ce522728400".into());
    m.insert(6, "71587193-271c-4098-f0e7-2f99b82ae100".into());
    m.insert(5, "94a75155-26ac-49b1-3e95-9ab325c97600".into());
    m.insert(4, "b867715a-f0b8-4264-defa-49d467ee9300".into());
    m.insert(3, "0746d14c-a396-4aac-7afb-367c8b5d3700".into());
    m.insert(2, "15efa158-c92e-4f19-cdc6-4db47aa8f100".into());
    m.insert(1, "fa3d3d61-9b60-4563-5eea-4fca499a1200".into());

    m
});

pub static CATEGORIES: LazyLock<HashMap<usize, String>> = LazyLock::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(1, "portfolio".into());
    m.insert(2, "diptych".into());
    m.insert(3, "fauna".into());
    m.insert(4, "flora".into());
    m.insert(5, "landscape".into());
    m.insert(6, "live!".into());
    m.insert(7, "mode de vie".into());
    m.insert(8, "oiseaux".into());
    m
});

pub static PHOTOS_BY_CATEGORY: LazyLock<HashMap<usize, GalleryViewModel>> = LazyLock::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    let portfolio = json! ({ "id": 1,
      "name": "portfolio",
      "photos": [
        {
          "id": 114,
          "title": "circlet",
          "filename": "circlet.jpg",
          "location_taken": "deer creek heights, wyoming",
          "date_taken": "2017-08-21"
        },
        {
          "id": 115,
          "title": "moose and tetons",
          "filename": "america_moose.jpg",
          "location_taken": "Jackson, Wyoming",
          "date_taken": "2015-12-28"
        },
        {
          "id": 116,
          "title": "mjolnir",
          "filename": "thor_s_hammer.jpg",
          "location_taken": "Sunset Trail near Queen's Garden, Bryce Canyon National Park, Utah",
          "date_taken": "2014-03-14"
        },
        {
          "id": 117,
          "title": "on jackson lake",
          "filename": "jackson_lake.jpg",
          "location_taken": "Jackson Lake, Grand Teton National Park, Wyoming",
          "date_taken": "2013-07-02"
        },
        {
          "id": 118,
          "title": "time traveller",
          "filename": "time_traveller.jpg",
          "location_taken": "Ashburn, Virginia",
          "date_taken": "2013-06-08"
        },
        {
          "id": 119,
          "title": "a home with a view",
          "filename": "a_home_with_a_view.jpg",
          "location_taken": "Teton Village, Wyoming",
          "date_taken": "2012-07-11"
        },
        {
          "id": 120,
          "title": "seal on the rocks",
          "filename": "seal.jpg",
          "location_taken": "La Jolla, California",
          "date_taken": "2010-12-23"
        },
        {
          "id": 121,
          "title": "patio",
          "filename": "under.jpg",
          "location_taken": "Lyons, Colorado",
          "date_taken": "2010-06-27"
        },
        {
          "id": 122,
          "title": "solitary western monk's hood",
          "filename": "western_monk_s_hood.jpg",
          "location_taken": "Laurance S Rockefeller Preserve, Grand Teton National Park, Wyoming",
          "date_taken": "2009-08-25"
        },
        {
          "id": 123,
          "title": "eclipse",
          "filename": "eclipse.jpg",
          "location_taken": "deer creek heights, wyoming",
          "date_taken": "2017-08-21"
        }
      ]
    });

    m.insert(1, serde_json::from_value(portfolio).unwrap());
    m.insert(2, serde_json::from_value(json!( {"id":2,"name":"diptych","photos":[{"id":1,"title":"narrows - diptych","filename":"diptych.jpg","location_taken":"The Narrows, Zion National Park, Utah","date_taken":"2014-03-12"},{"id":2,"title":"narrows - diptych","filename":"diptych2.jpg","location_taken":"The Narrows Zion National Park, Utah","date_taken":"2014-03-12"}]})).unwrap());
    m.insert(3, serde_json::from_value(json!( {"id":3,"name":"fauna","photos":[{"id":3,"title":"moose in brush","filename":"camoflauge.jpg","location_taken":"Grand Teton National Park, Wyoming","date_taken":"2010-07-10"},{"id":4,"title":"seal on the rocks","filename":"seal.jpg","location_taken":"La Jolla, California","date_taken":"2010-12-23"},{"id":5,"title":"mountaineering big horn","filename":"mountaineering.jpg","location_taken":"Mammoth Hot Springs, Yellowstone, Wyoming","date_taken":"2012-07-12"},{"id":6,"title":"baby bighorn","filename":"baby_bighorn.jpg","location_taken":"Mammoth Hot Springs, Yellowstone, Wyoming","date_taken":"2012-07-12"},{"id":7,"title":"river otter","filename":"river_otter.jpg","location_taken":"Snake River, Jackson, Wyoming","date_taken":"2012-07-14"},{"id":8,"title":"deer in headlights","filename":"deer_in_headlights.jpg","location_taken":"Sunset Trail near Queen's Garden, Bryce Canyon National Park, Utah","date_taken":"2014-03-14"},{"id":9,"title":"rise","filename":"big_horn_sheep_2.jpg","location_taken":"Jackson, Wyoming","date_taken":"2014-03-08"},{"id":10,"title":"friends","filename":"friends.jpg","location_taken":"Flat Creek, Jackson, Wyoming","date_taken":"2013-07-12"},{"id":11,"title":"buffalo family","filename":"buffalo_family.jpg","location_taken":"Yellowstone National Park, Wyoming","date_taken":"2013-07-06"},{"id":12,"title":"american buffalo","filename":"american_buffalo.jpg","location_taken":"Yellowstone National Park, Wyoming","date_taken":"2013-07-06"},{"id":13,"title":"river otter family","filename":"river_otter_family.jpg","location_taken":"Snake River, Jackson, Wyoming","date_taken":"2012-07-14"}]})).unwrap());
    m.insert(4, serde_json::from_value(json!( {"id":4,"name":"flora","photos":[{"id":14,"title":"poppy","filename":"poppy.jpg","location_taken":"New Orleans Botanical Garden, Louisiana","date_taken":"2018-04-15"},{"id":15,"title":"new orleans botanical gardens water feature","filename":"fountain.jpg","location_taken":"New Orleans Botanical Garden, Louisiana","date_taken":"2018-04-15"},{"id":16,"title":"jardin","filename":"jardin.jpg","location_taken":"New Orleans Botanical Garden, Louisiana","date_taken":"2018-04-15"},{"id":17,"title":"solitary western monk's hood","filename":"western_monk_s_hood.jpg","location_taken":"Laurance S Rockefeller Preserve, Grand Teton National Park, Wyoming","date_taken":"2009-08-24"},{"id":18,"title":"frozen dew","filename":"icicle.jpg","location_taken":"Goose Creek, Lansdowne, Virginia","date_taken":"2009-01-14"},{"id":19,"title":"a world on the pedal of a flower","filename":"the_world_on_the_pedal_of_a_flower.jpg","location_taken":"Laurance S Rockefeller Preserve, Grand Teton National Park, Wyoming","date_taken":"2008-08-08"},{"id":20,"title":"solitude","filename":"solitude.jpg","location_taken":"Jackson, Wyoming","date_taken":"2008-08-03"}]})).unwrap());
    m.insert(5, serde_json::from_value(json!( {"id":5,"name":"landscape","photos":[{"id":21,"title":"sunset","filename":"sunset_2.jpg","location_taken":"Moonlight Beach, California","date_taken":"2008-12-23"},{"id":22,"title":"coastal","filename":"coastal.jpg","location_taken":"La Jolla, California","date_taken":"2008-12-21"},{"id":23,"title":"potomac River at Sunrise","filename":"potomac_river.jpg","location_taken":"Potomac River, Lansdowne, Virginia","date_taken":"2008-10-04"},{"id":24,"title":"d'or","filename":"d_or.jpg","location_taken":"Potomac River, Lansdowne, Virginia","date_taken":"2008-10-04"},{"id":25,"title":"foggy morning spider's web","filename":"web.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2008-09-30"},{"id":26,"title":"geothermal","filename":"geothermal.jpg","location_taken":"Yellowstone National Park, Wyoming","date_taken":"2008-08-18"},{"id":27,"title":"inspiration point","filename":"inspiration_point.jpg","location_taken":"Cascade Canyon, Grand Teton National Park, Wyoming","date_taken":"2008-08-11"},{"id":28,"title":"pastel","filename":"pastel.jpg","location_taken":"Moose Wilson Road Overlook, Grand Teton National Park, Wyoming","date_taken":"2008-08-02"},{"id":29,"title":"goose creek","filename":"goose_creek.jpg","location_taken":"Goose Creek, Lansdowne, Virginia","date_taken":"2009-01-14"},{"id":30,"title":"setting sun","filename":"sebego.jpg","location_taken":"Sebego Lake, Maine","date_taken":"2009-08-08"},{"id":31,"title":"crash:tide","filename":"crash-tide.jpg","location_taken":"La Jolla, California","date_taken":"2010-12-23"},{"id":32,"title":"nola skyline","filename":"nola.jpg","location_taken":"New Orleans, Louisiana","date_taken":"2018-04-13"},{"id":33,"title":"total eclipse of the sun","filename":"totality.jpg","location_taken":"Deer Creek Heights, Wyoming","date_taken":"2017-08-21"},{"id":34,"title":"artist's point","filename":"artist_s_point.jpg","location_taken":"Yellowstone National Park, Wyoming","date_taken":"2016-07-06"},{"id":35,"title":"sister and the grand tetons","filename":"grand_and_d.jpg","location_taken":"Jackson, Wyoming","date_taken":"2015-12-28"},{"id":36,"title":"moose and tetons","filename":"america_moose.jpg","location_taken":"Jackson, Wyoming","date_taken":"2015-12-28"},{"id":37,"title":"mjolnir","filename":"thor_s_hammer.jpg","location_taken":"Sunset Trail near Queen's Garden, Bryce Canyon National Park, Utah","date_taken":"2014-03-14"},{"id":38,"title":"between sun & moon","filename":"between_sun_&_moon.jpg","location_taken":"Sunset Trail near Queen's Garden, Bryce Canyon National Park, Utah","date_taken":"2014-03-13"},{"id":39,"title":"hoodoos","filename":"hoodoos_2.jpg","location_taken":"Sunrise Point, Bryce Canyon National Park, Utah","date_taken":"2014-03-13"},{"id":40,"title":"hoodoos","filename":"bryce.jpg","location_taken":"Sunrise Point, Bryce Canyon National Park, Utah","date_taken":"2014-03-13"},{"id":41,"title":"ravine","filename":"ravine.jpg","location_taken":"Sand Dune Arch, Arches National Park, Utah","date_taken":"2014-03-13"},{"id":42,"title":"tower of babel","filename":"tower_of_babel.jpg","location_taken":"Courthouse Towers Viewpoint, Arches National Park, Utah","date_taken":"2014-03-13"},{"id":43,"title":"parade of elephants","filename":"parade_of_elephants.jpg","location_taken":"Arches National Park, Utah","date_taken":"2014-03-13"},{"id":44,"title":"tunnel","filename":"tunnel_2.jpg","location_taken":"The Narrows, Zion National Park, Utah","date_taken":"2014-03-12"},{"id":45,"title":"subterranean","filename":"subteranean.jpg","location_taken":"The Narrows, Zion National Park, Utah","date_taken":"2014-03-12"},{"id":46,"title":"starlit sinawava temple","filename":"starlit_sinawava_temple.jpg","location_taken":"The Narrows, Zion National Park, Utah","date_taken":"2014-03-12"},{"id":47,"title":"river & rock","filename":"river_&_rock.jpg","location_taken":"The Narrows, Zion National Park, Utah","date_taken":"2014-03-12"},{"id":48,"title":"river","filename":"rver.jpg","location_taken":"The Narrows, Zion National Park, Utah","date_taken":"2014-03-12"},{"id":49,"title":"oxbow bend","filename":"oxbow_bend.jpg","location_taken":"Oxbow Bend, Grand Teton National Park, Wyoming","date_taken":"2013-07-06"},{"id":50,"title":"on jackson lake","filename":"jackson_lake.jpg","location_taken":"Jackson Lake, Grand Teton National Park, Wyoming","date_taken":"2013-07-02"},{"id":51,"title":"ground at mammoth","filename":"edge_of_earth.jpg","location_taken":"Mammoth Hot Springs, Yellowstone, Wyoming","date_taken":"2012-07-12"},{"id":52,"title":"cry lightning, the delicate sound of thunder","filename":"cry_lightning_2.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2010-05-27"}]})).unwrap());
    m.insert(6, serde_json::from_value(json!( {"id":6,"name":"live!","photos":[{"id":53,"title":"burke of the concussion theory","filename":"burke.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-10-16"},{"id":54,"title":"the concussion theory","filename":"concert_3.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-05-03"},{"id":55,"title":"the concussion theory","filename":"concert_2.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-05-03"},{"id":56,"title":"the concussion theory","filename":"concert_1.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-05-03"},{"id":57,"title":"nate of the concussion theory","filename":"nate_tct.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-03-21"},{"id":58,"title":"nate of the concussion theory","filename":"nate.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-03-21"},{"id":59,"title":"the concussion theory","filename":"tct.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-03-02"}]})).unwrap());
    m.insert(7, serde_json::from_value(json!( {"id":7,"name":"mode de vie","photos":[{"id":60,"title":"concrete","filename":"concrete.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2008-12-05"},{"id":61,"title":"zen","filename":"zen.jpg","location_taken":"New Orleans Botanical Garden, Louisiana","date_taken":"2018-04-15"},{"id":62,"title":"sawyer and kirsten","filename":"couple.jpg","location_taken":"New Orleans Botanical Garden, Louisiana","date_taken":"2018-04-15"},{"id":63,"title":"so grey, so blue","filename":"so_grey,_so_blue.jpg","location_taken":"Ashburn, Virginia","date_taken":"2010-05-30"},{"id":64,"title":"light trails","filename":"moonrise,_thoughtful_eyese.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2008-12-01"},{"id":65,"title":"circlet","filename":"circlet.jpg","location_taken":"deer creek heights, wyoming","date_taken":"2017-08-21"},{"id":66,"title":"precursor to a decade","filename":"79.jpg","location_taken":"Leesburg, Virginia","date_taken":"2016-08-11"},{"id":67,"title":"bmw","filename":"bmw.jpg","location_taken":"Ashburn, Virginia","date_taken":"2016-08-11"},{"id":68,"title":"megan","filename":"megan_4.jpg","location_taken":"Studio","date_taken":"2015-07-21"},{"id":69,"title":"the concussion theory","filename":"the_concussion_theory_10.jpg","location_taken":"Richmond, Virginia","date_taken":"2014-05-02"},{"id":70,"title":"time traveller","filename":"time_traveller.jpg","location_taken":"Ashburn, Virginia","date_taken":"2013-06-08"},{"id":71,"title":"american dream","filename":"american_dream_1.jpg","location_taken":"Ashburn, Virginia","date_taken":"2013-06-08"},{"id":72,"title":"longboarding","filename":"longboarding_2.jpg","location_taken":"Belmont Country Club, Ashburn, Virginia","date_taken":"2013-05-29"},{"id":73,"title":"zach hottle wallride","filename":"wallride.jpg","location_taken":"Ashburn, Virginia","date_taken":"2013-04-13"},{"id":74,"title":"richmond skyline","filename":"richmond_skyline.jpg","location_taken":"Richmond, Virginia","date_taken":"2012-12-11"},{"id":75,"title":"contrast","filename":"contrast.jpg","location_taken":"Ashburn, Virginia","date_taken":"2012-04-28"},{"id":76,"title":"cascading drops of coloured water","filename":"concept.jpg","location_taken":"Studio","date_taken":"2012-02-16"},{"id":77,"title":"with nothing to offer except confusion…","filename":"drink.jpg","location_taken":"Studio","date_taken":"2012-01-05"},{"id":78,"title":"roads less traveled","filename":"roads_less_traveled.jpg","location_taken":"Ashburn, Virginia","date_taken":"2011-12-03"},{"id":79,"title":"elevator","filename":"elevator.jpg","location_taken":"Minneapolis, Mn","date_taken":"2011-11-18"},{"id":80,"title":"music to my ears","filename":"music_to_my_ears_5.jpg","location_taken":"Ashburn, Virginia","date_taken":"2011-08-16"},{"id":81,"title":"bad news","filename":"bad_news.jpg","location_taken":"Ashburn, Virginia","date_taken":"2011-05-13"},{"id":82,"title":"self realization center gardens","filename":"self_realization.jpg","location_taken":"Self-Realization Fellowship Hermitage, Encinitas, California","date_taken":"2010-12-26"},{"id":83,"title":"stage","filename":"old_town_3.jpg","location_taken":"Old Town San Diego, California","date_taken":"2010-12-24"},{"id":84,"title":"old town","filename":"old_town.jpg","location_taken":"Old Town San Diego, California","date_taken":"2010-12-24"},{"id":85,"title":"mission","filename":"mission.jpg","location_taken":"Old Town San Diego, California","date_taken":"2010-12-24"},{"id":86,"title":"nightlife","filename":"nightlife.jpg","location_taken":"Ashburn, Virginia","date_taken":"2010-11-04"},{"id":87,"title":"nobody's hero","filename":"nobody_s_hero_2.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2010-10-10"},{"id":88,"title":"apple valley farms","filename":"koi.jpg","location_taken":"Lyons, Colorado","date_taken":"2010-06-30"},{"id":89,"title":"apple valley farms","filename":"under.jpg","location_taken":"Lyons, Colorado","date_taken":"2010-06-27"},{"id":90,"title":"late for work ","filename":"commute.jpg","location_taken":"Ashburn, Virginia","date_taken":"2010-06-04"},{"id":91,"title":"just another day at the office","filename":"just_another_day_at_the_office_6.jpg","location_taken":"Ashburn, Virginia","date_taken":"2010-06-04"},{"id":92,"title":"megan","filename":"megan_1.jpg","location_taken":"Studio","date_taken":"2017-12-01"},{"id":93,"title":"just another day at the office","filename":"just_another_day_at_the_office_2.jpg","location_taken":"Ashburn, Virginia","date_taken":"2010-06-04"},{"id":94,"title":"digital sage","filename":"digital_sage.jpg","location_taken":"New Orleans, Louisiana","date_taken":"2018-04-13"},{"id":95,"title":"succulents","filename":"kris.jpg","location_taken":"New Orleans Botanical Garden, Louisiana","date_taken":"2018-04-15"}]})).unwrap());
    m.insert(8, serde_json::from_value(json!( {"id":8,"name":"oiseaux","photos":[{"id":96,"title":"takeoff","filename":"takeoff.jpg","location_taken":"Jackson, Wyoming","date_taken":"2014-03-09"},{"id":97,"title":"racing reflections","filename":"racing_reflections.jpg","location_taken":"Flat Creek, Jackson, Wyoming","date_taken":"2013-07-12"},{"id":98,"title":"friends","filename":"friends.jpg","location_taken":"Flat Creek, Jackson, Wyoming","date_taken":"2013-07-12"},{"id":99,"title":"canary in a charcoal mine","filename":"canary_in_a_charcoal_mine.jpg","location_taken":"Jackson Lake, Grand Teton National Park, Wyoming","date_taken":"2013-07-11"},{"id":100,"title":"nest egg","filename":"nest_egg.jpg","location_taken":"Teton Village, Wyoming","date_taken":"2012-07-11"},{"id":101,"title":"a home with a view","filename":"a_home_with_a_view.jpg","location_taken":"Teton Village, Wyoming","date_taken":"2012-07-10"},{"id":102,"title":"cooling off","filename":"swim.jpg","location_taken":"Jackson Lake, Grand Teton National Park, Wyoming","date_taken":"2011-07-24"},{"id":103,"title":"warbler","filename":"warbler_2.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2011-03-08"},{"id":104,"title":"jonathan livingston seagull","filename":"jonathan_livingston_seagull.jpg","location_taken":"Coronado Island, California","date_taken":"2010-12-28"},{"id":105,"title":"hummingbird","filename":"hummingbird.jpg","location_taken":"Aviara Trail, San Diego, California","date_taken":"2010-12-26"},{"id":106,"title":"landing","filename":"landing_2.jpg","location_taken":"La Jolla, California","date_taken":"2010-12-23"},{"id":107,"title":"icarus","filename":"icarus.jpg","location_taken":"La Jolla, California","date_taken":"2010-12-21"},{"id":108,"title":"great blue heron on ice covered pond","filename":"the_thin_ice.jpg","location_taken":"Lansdowne Golf Course, Lansdowne, Virginia","date_taken":"2009-01-25"},{"id":109,"title":"perch","filename":"perch_1_2.jpg","location_taken":"Lansdowne, Virginia","date_taken":"2009-01-19"},{"id":110,"title":"insight","filename":"insight.jpg","location_taken":"La Jolla, California","date_taken":"2008-12-21"},{"id":111,"title":"female western bluebird","filename":"flight_2.jpg","location_taken":"Morning Glory Trail - Old Faithful Area, Upper Geyser Basin - Yellowstone, Wyoming","date_taken":"2008-08-18"},{"id":112,"title":"female western bluebird","filename":"flight.jpg","location_taken":"Morning Glory Trail - Old Faithful Area, Upper Geyser Basin - Yellowstone, Wyoming","date_taken":"2008-08-18"},{"id":113,"title":"perch","filename":"perch.jpg","location_taken":"Snake River Overlook, Hoback Junction, Wyoming","date_taken":"2008-08-17"}]})).unwrap());
    m
});

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct CategoryViewModel {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PhotoViewModel {
    pub id: usize,
    pub title: String,
    pub filename: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    // pub cloudflare_resource: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct PhotoViewModel2 {
    pub id: usize,
    pub title: String,
    pub filename: String,
    pub location_taken: String,
    pub date_taken: NaiveDate,
    pub cloudflare_resource: String,
}

impl From<PhotoViewModel> for PhotoViewModel2 {
    fn from(value: PhotoViewModel) -> Self {
        PhotoViewModel2 {
            id: value.id,
            title: value.title,
            filename: value.filename,
            location_taken: value.location_taken,
            date_taken: value.date_taken,
            cloudflare_resource: format!(
                "https://imagedelivery.net/ubbJTaQO-oc8x-62MYWhKg/{}/public",
                CLOUDFLARE_RESOURCES.get(&value.id).unwrap()
            ),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GalleryViewModel {
    pub photos: Vec<PhotoViewModel>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GalleryViewModel2 {
    pub photos: Vec<PhotoViewModel2>,
}

impl From<GalleryViewModel> for GalleryViewModel2 {
    fn from(value: GalleryViewModel) -> Self {
        Self {
            photos: value.photos.into_iter().map(|v| v.into()).collect(),
        }
    }
}

pub async fn get_photos(category_id: usize) -> GalleryViewModel2 {
    PHOTOS_BY_CATEGORY
        .get(&(category_id))
        .unwrap()
        .clone()
        .into()
}

pub async fn get_photo(photo_id: usize) -> PhotoViewModel2 {
    PHOTOS_BY_CATEGORY
        .iter()
        .flat_map(|(_, v)| v.photos.clone())
        .find(|v| v.id == photo_id)
        .unwrap()
        .into()
}

pub async fn get_categories() -> Vec<CategoryViewModel> {
    CATEGORIES
        // .clone()
        .iter()
        .map(|(id, name)| CategoryViewModel {
            id: *id,
            name: name.clone(),
        })
        .collect()
}

/*

pub async fn get_photo(photo_id: usize) -> PhotoViewModel {
    let url = format!("http://127.0.0.1:6969/api/v0/photos/{}", photo_id);
    // let resp = reqwest::get(url).await.unwrap();
    let resp = Client::new().get(url).send().await.unwrap();
    // let resp = Client::new()::get(url).await.unwrap();
    let photo: PhotoViewModel = resp.json().await.unwrap();
    photo
}

pub async fn get_categories() -> Vec<CategoryViewModel> {
    let url = "http://127.0.0.1:6969/api/v0/categories";
    // let resp = reqwest::get(url).await.unwrap();
    let resp = Client::new().get(url).send().await.unwrap();
    // let resp = Client::new()::get(url).await.unwrap();
    let categories: Vec<CategoryViewModel> = resp.json().await.unwrap();
    categories
}

pub async fn get_photos(category_id: usize) -> GalleryViewModel {
    let url = format!("http://127.0.0.1:6969/api/v0/categories/{}", category_id);
    // let resp = reqwest::get(url).await.unwrap();
    let resp = Client::new().get(url).send().await.unwrap();
    // let resp = Client::new()::get(url).await.unwrap();
    let photos: GalleryViewModel = resp.json().await.unwrap();
    dbg!(&photos);
    photos
}
*/
