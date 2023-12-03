using System.Text.RegularExpressions;
using AdventOfCode.Libs;

namespace AdventOfCode.Y2023.D02;

[ProblemName("Cube Conundrum")]
public class Solution(string input) : Solver(input)
{
    [ProblemSolution("1867")]
    public override string Solve1()
    {
        var count = GetLines()
            .Select(line =>
            {
                var segments = line.Split(": ");
                var gameId = int.Parse(Regex.Match(segments[0], @"Game (\d{1,})").Groups[1].Value);
                var games = segments[1].Split("; ");

                var results = games
                    .Select(game => new
                    {
                        Red = GetCount(game, "red"),
                        Green = GetCount(game, "green"),
                        Blue = GetCount(game, "blue")
                    })
                    .Aggregate((acc, item) => new
                    {
                        Red = Math.Max(acc.Red, item.Red),
                        Green = Math.Max(acc.Green, item.Green),
                        Blue = Math.Max(acc.Blue, item.Blue),
                    });
                
                return results.Red <= 12 && results.Green <= 13 && results.Blue <= 14 
                    ? gameId 
                    : 0;
            })
            .Sum();
        
        return count.ToString();
    }

    public override string Solve2()
    {
        var count = GetLines()
            .Select(line =>
            {
                var segments = line.Split(": ");
                var gameId = int.Parse(Regex.Match(segments[0], @"Game (\d{1,})").Groups[1].Value);
                var games = segments[1].Split("; ");

                var results = games
                    .Select(game => new
                    {
                        Red = GetCount(game, "red"),
                        Green = GetCount(game, "green"),
                        Blue = GetCount(game, "blue")
                    })
                    .Aggregate((acc, item) => new
                    {
                        Red = Math.Max(acc.Red, item.Red),
                        Green = Math.Max(acc.Green, item.Green),
                        Blue = Math.Max(acc.Blue, item.Blue),
                    });
                
                return results.Red * results.Green * results.Blue;
            })
            .Sum();
        
        return count.ToString();
    }

    private int GetCount(string game, string color)
    {
        var groups = Regex.Match(game, @"(\d{1,}) " + color).Groups;
        return int.Parse(groups.Count > 1 ? groups[1].Value : "0");
    }
}