use std::collections::HashMap;

mod time;
mod types;

fn main() {
    run()
}

fn run() {
    // ディレクトリ配下のすべてのGames読み込み
    let paths = std::fs::read_dir("./data/").unwrap();

    let mut games: Vec<types::Game> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap();
        // jsonファイル読み込み
        let file = std::fs::read_to_string(path_str).unwrap();
        // jsonファイルをパース
        let games_root: types::GamesRoot = serde_json::from_str(&file).unwrap();

        // games_root.gamesをgamesに追加
        games_root.games.iter().for_each(|game| {
            games.push(game.clone());
        });
    }

    // 昔のゲームから順にソート
    games.sort_by_key(|game| game.created_time.seconds);

    // 集計期間を指定
    let start_seconds = time::get_unix_time(2016);
    let end_seconds = time::get_unix_time(2023);

    let games = games
        .iter()
        .filter(|game| {
            let s = game.created_time.seconds;
            s >= start_seconds && s < end_seconds
        })
        .collect::<Vec<&types::Game>>();

    // プレイヤー全員のレートを1500で初期化
    let mut players_rate = HashMap::new();

    games.iter().for_each(|game| {
        for player in game.players.iter() {
            players_rate.insert(player.clone(), 1500.0);
        }
    });

    // 順位ポイント
    let rank_point = vec![50.0, 10.0, -20.0, -40.0];

    // 全半荘でレートを計算
    games.iter().for_each(|game| {
        for m in &game.matches {
            // 同卓者の名前一覧を取得
            let players = m.keys().collect::<Vec<&String>>();

            // 同卓者の平均レートを計算
            let avg_rate: f64 = players
                .iter()
                .map(|player| players_rate.get(*player).unwrap())
                .sum::<f64>()
                / players.len() as f64;

            // 同卓者の着順で降順ソートしておく
            let mut v: Vec<(String, f64)> = m.iter().map(|(k, v)| (k.clone(), *v)).collect();
            v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            let v = v;

            // 個人のレートを計算
            v.iter().enumerate().for_each(|(i, (player, _))| {
                let old_rate = players_rate.get(player).unwrap();
                let diff = (rank_point[i] + (avg_rate - old_rate) / 40.0) * 0.2;
                let new_rate = old_rate + diff;
                players_rate.insert(player.clone(), new_rate);

                print!("{:}: {:+2.1} ", player, diff);
            });
            print!(" / 卓平均レート: {:0.1}", avg_rate);
            println!();
        }
    });

    // レートの高い順にソート
    let mut result = players_rate
        .iter()
        .map(|(k, v)| (k, v.round() as i64))
        .collect::<Vec<(&String, i64)>>();

    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // 最終結果表示
    result.iter().enumerate().for_each(|(i, (player, rate))| {
        println!("{}位: {}({})", i + 1, player, rate);
    });
}
