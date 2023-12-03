namespace AdventOfCode.Libs;

public abstract class Solver(string input)
{
    protected readonly string Input = input;

    public string[] GetLines()
    {
        return Input.Replace("\r", "").Split('\n');
    }
    
    public virtual string Solve1()
    {
        return string.Empty;
    }
    
    public virtual string Solve2()
    {
        return string.Empty;
    }
    
    
}