{-# LANGUAGE DuplicateRecordFields #-}

-- Define operations as type classes
class Area a where area :: a -> Double
class Perimeter a where perimeter :: a -> Double

-- Define data types with overlapping field names
data Circle = Circle { radius :: Double }
data Rectangle = Rectangle { width :: Double, height :: Double }
data Triangle = Triangle { base :: Double, height :: Double }

-- Implement operations
instance Area Circle where area (Circle r) = pi * r^2
instance Perimeter Circle where perimeter (Circle r) = 2 * pi * r

instance Area Rectangle where area (Rectangle w h) = w * h
instance Perimeter Rectangle where perimeter (Rectangle w h) = 2 * (w + h)

instance Area Triangle where area (Triangle b h) = 0.5 * b * h
instance Perimeter Triangle where perimeter (Triangle b h) = b + 2 * sqrt ((b/2)^2 + h^2)

-- Heterogeneous list using existential quantification
data Shape = forall a. (Area a, Perimeter a) => Shape a

main :: IO ()
main = mapM_ (\(Shape s) -> print (area s, perimeter s)) shapes
  where
    shapes = [Shape (Circle 3), Shape (Rectangle 4 5), Shape (Triangle 6 4)]
