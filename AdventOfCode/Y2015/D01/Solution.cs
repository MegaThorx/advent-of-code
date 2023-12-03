using System.Text.RegularExpressions;
using AdventOfCode.Libs;

namespace AdventOfCode.Y2015.D01;

[ProblemName("Not Quite Lisp")]
public class Solution(string input) : Solver(input)
{
    [ProblemSolution("138")]
    public override string Solve1()
    {
        return (Input.Count(x => x == '(') - Input.Count(x => x == ')')).ToString();
    }

    [ProblemSolution("1771")]
    public override string Solve2()
    {
        var floor = 0;
        return (Input.TakeWhile(character => (floor += character == '(' ? 1 : -1) != -1).Count() + 1).ToString();
    }
}