namespace AdventOfCode.Libs;

public class ProblemSolutionAttribute(string solution) : Attribute
{
    public string Solution { get; } = solution;
}