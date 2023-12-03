using System.Reflection;
using System.Text.RegularExpressions;
using AdventOfCode.Libs;
using BetterConsoleTables;

namespace AdventOfCode;

public class Program
{
    public static void Main()
    {
        var solvers = typeof(Program).Assembly.GetTypes()
            .Where(x => typeof(Solver).IsAssignableFrom(x) && x.FullName.StartsWith("AdventOfCode.Y"))
            .Select(x => new Solution(
                x.GetCustomAttribute<ProblemNameAttribute>(true)?.Name ?? string.Empty,
                x,
                $"{x.Namespace?.Replace("AdventOfCode.", "").Replace('.', '/')}/input.txt",
                int.Parse(Regex.Match(x.Namespace ?? string.Empty, @"\.Y(\d{4})\.").Groups[1].Value),
                int.Parse(Regex.Match(x.Namespace ?? string.Empty, @"\.D(\d{2})").Groups[1].Value)
            )).ToList();



        var table = new Table("Year", "Day", "Task 1", "Solution 1", "Task 2", "Solution 2");
        table.Config = TableConfiguration.Unicode();
        
        var years = solvers.Select(x => x.Year).Distinct().Order();

        foreach (var year in years)
        {
            foreach (var solver in solvers.Where(x => x.Year == year).OrderBy(x => x.Day))
            {
                var instance = Activator.CreateInstance(solver.Type, File.ReadAllText(solver.Path)) as Solver;
                
                if (instance is not null)
                {
                    var solution1 =
                        solver.Type.GetMethod("Solve1")?.GetCustomAttribute<ProblemSolutionAttribute>(true)?.Solution ??
                        string.Empty;
                    var solution2 =
                        solver.Type.GetMethod("Solve2")?.GetCustomAttribute<ProblemSolutionAttribute>(true)?.Solution ??
                        string.Empty;

                    table.AddRow(year, solver.Day, instance.Solve1(), solution1,  instance.Solve2(), solution2);
                }
            }
        }
        
        Console.Write(table.ToString());
    }
}