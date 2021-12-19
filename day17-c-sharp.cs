using System;
using System.Collections.Generic;
using System.Linq;

namespace adventofcode_cs
{
    internal struct Vec2
    {
        public int X, Y;

        public override string ToString() => $"[{X}, {Y}]";

        public static Vec2 operator +(Vec2 a, Vec2 b) => new() {X = a.X + b.X, Y = a.Y + b.Y};
        public static Vec2 operator -(Vec2 a, Vec2 b) => new() {X = a.X - b.X, Y = a.Y - b.Y};

        public static Vec2 Zero => new() {X = 0, Y = 0};
    }

    internal struct TargetArea
    {
        public Vec2 Min;
        public Vec2 Max;

        public override string ToString() =>
            $"TargetArea {{ Min: {Min}, Max: {Max} }}";

        public static TargetArea Parse(string input)
        {
            var coords = input
                .Replace("target area:", "").Trim().Split(",")
                .Select(a => a.Split("=")[1].Trim().Split(".."))
                .ToList();

            return new TargetArea()
            {
                Min = new Vec2() {X = Convert.ToInt32(coords[0][0]), Y = Convert.ToInt32(coords[1][0])},
                Max = new Vec2() {X = Convert.ToInt32(coords[0][1]), Y = Convert.ToInt32(coords[1][1])},
            };
        }
    }

    internal struct Trajectory
    {
        public Vec2 Position;
        public Vec2 Velocity;

        public Trajectory(Vec2 position, Vec2 velocity)
        {
            Position = position;
            Velocity = velocity;
        }

        public void Simulate()
        {
            Position += Velocity;
            Velocity.X -= Velocity.X == 0 ? 0 : Velocity.X > 0 ? 1 : -1;
            Velocity.Y -= 1;
        }

        public (bool, int) SimulateUntilTarget(TargetArea target)
        {
            var maxHeight = Position.Y;

            for (var i = 0;; i++)
            {
                maxHeight = Math.Max(maxHeight, Position.Y);
                
                if (Position.X >= target.Min.X && 
                    Position.Y >= target.Min.Y && 
                    Position.X <= target.Max.X && 
                    Position.Y <= target.Max.Y)
                    return (true, maxHeight); // Target reached
                
                if (Position.X > target.Max.X || Position.Y < target.Min.Y)
                    return (false, maxHeight); // Overshot
                
                Simulate();
            }
        }

        public static Trajectory FromOrigin(Vec2 velocity)
        {
            return new(Vec2.Zero, velocity);
        }
    }

    class Day17
    {
        private static void Main(string[] args)
        {
            var input = "target area: x=137..171, y=-98..-73";
            // var input = "target area: x=20..30, y=-10..-5";
            
            var target = TargetArea.Parse(input);

            var maxHeight = 0;
            var validVelocities = new HashSet<Vec2>();
                
            // Search range could be improved
            for (var x = 0; x <= target.Max.X; x++)
            {
                for (var y = -1000; y < 1000; y++)
                {
                    var velocity = new Vec2() {X = x, Y = y};
                    
                    var (hit, height) =
                        Trajectory
                            .FromOrigin(velocity)
                            .SimulateUntilTarget(target);

                    if (hit)
                    {
                        maxHeight = Math.Max(maxHeight, height);
                        validVelocities.Add(velocity);
                    }
                }
            }
            
            Console.WriteLine($"Task 1: {maxHeight}");
            Console.WriteLine($"Task 2: {validVelocities.Count}");
        }
    }
}
