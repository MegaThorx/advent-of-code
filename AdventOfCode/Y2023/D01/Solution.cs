using System.Text.RegularExpressions;
using AdventOfCode.Libs;

namespace AdventOfCode.Y2023.D01;

[ProblemName("Trebuchet?!")]
public class Solution(string input) : Solver(input)
{
    [ProblemSolution("55108")]
    public override string Solve1()
    {
        return Match(@"\d").ToString();
    }

    [ProblemSolution("56324")]
    public override string Solve2()
    {
        return Match(@"\d|one|two|three|four|five|six|seven|eight|nine").ToString();
    }

    private int Match(string regex)
    {
        return (from line in GetLines()
            let first = Regex.Match(line, regex).Value
            let last = Regex.Match(line, regex, RegexOptions.RightToLeft).Value
            select ParseValue(first) * 10 + ParseValue(last)).Sum();
    }

    private int ParseValue(string value) => value switch
    {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => int.Parse(value)
    };
}