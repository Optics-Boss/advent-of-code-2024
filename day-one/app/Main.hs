module Main where

parseToTwoArrays :: String -> (Int, Int)
parseToTwoArrays line = (number1, number2)
  where (read number1, read number2) -> break (== ' ') line

main :: IO ()
main = do
  print $ parseToTwoArrays . lines <$> readFile "input.txt"
  -- let emptyList = [1,2,3,4,5,6]
  -- let sortedList = sort.emptyList
  -- print sortedList
