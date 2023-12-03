namespace AdventOfCode.Libs;

public class ProblemNameAttribute(string name) : Attribute
{
    public string Name { get; } = name;
}