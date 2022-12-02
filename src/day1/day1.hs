import Data.List

sampleData :: String
sampleData =
    "1000\n\
    \2000\n\
    \3000\n\
    \\n\
    \4000\n\
    \\n\
    \5000\n\
    \6000\n\
    \\n\
    \7000\n\
    \8000\n\
    \9000\n\
    \\n\
    \10000"

solve1 :: String -> Integer
solve1 =
    maximum
        . map (sum . map read)
        . filter (\x -> x /= [""])
        . groupBy (\a b -> a /= "" && b /= "")
        . lines

solve2 :: String -> Integer
solve2 =
    sum . take 3
        . sortBy (flip compare)
        . map (sum . map read)
        . filter (\x -> x /= [""])
        . groupBy (\a b -> a /= "" && b /= "")
        . lines
