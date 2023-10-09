pub fn levenshtein_distance(str1: &str, str2: &str) -> usize {
    let len1 = str1.chars().count();
    let len2 = str2.chars().count();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else {
                let cost = if str1.chars().nth(i - 1) != str2.chars().nth(j - 1) {
                    1
                } else {
                    0
                };
                dp[i][j] = std::cmp::min(
                    dp[i - 1][j] + 1,
                    std::cmp::min(dp[i][j - 1] + 1, dp[i - 1][j - 1] + cost),
                );
            }
        }
    }

    dp[len1][len2]
}

pub fn similarity_percentage(str1: &str, str2: &str) -> f64 {
    let distance = levenshtein_distance(str1, str2);
    let len1 = str1.chars().count();
    let len2 = str2.chars().count();

    let max_len = std::cmp::max(len1, len2) as f64;
    let similarity = 1.0 - (distance as f64 / max_len);

    similarity * 100.0
}
