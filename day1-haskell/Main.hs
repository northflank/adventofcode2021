import InputData

measureIncreases :: [Int] -> Int
measureIncreases (x:x2:xs)
  | x2 > x = acc + 1
  | otherwise = acc
  where acc = measureIncreases (x2 : xs)
measureIncreases (x:xs) = 0
measureIncreases [] = 0

measureIncRolling :: [Int] -> (Int, Int)
measureIncRolling (x1:x2:x3:x4:xs)
  | prevSum > newSum = (newSum, acc + 1)
  | otherwise = (newSum, acc)
  where newSum = x1 + x2 + x3
        (prevSum, acc) = measureIncRolling(x2:x3:x4:xs)
measureIncRolling (x1:x2:x3:xs) = (x1 + x2 + x3, 0)
measureIncRolling (x:xs) = (0, 0)
measureIncRolling [] = (0, 0)

main = do
  putStrLn "Part 1:"
  putStrLn (show (measureIncreases InputData.input))
  putStrLn "Part 2:"
  putStrLn (show (snd (measureIncRolling InputData.input)))
