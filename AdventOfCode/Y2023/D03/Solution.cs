using AdventOfCode.Libs;

namespace AdventOfCode.Y2023.D03;

public class Solution(string input) : Solver(input)
{
    public override string Solve1()
    {
        var lines = GetLines();
        var sum = 0;
        
        for (var lineNumber = 0; lineNumber < lines.Length; lineNumber++)
        {
            var line = lines[lineNumber];

            var currentlyOnDigit = false;
            var isPartNumber = false;
            var numbers = "";
            
            for (var characterNumber = 0; characterNumber < line.Length; characterNumber++)
            {
                if (char.IsDigit(line[characterNumber]))
                {
                    numbers += line[characterNumber];
                    if (IsPartNumber(lines, lineNumber, characterNumber))
                        isPartNumber = true;
                }
                else
                {
                    if (isPartNumber)
                        sum += int.Parse(numbers);
                    
                    isPartNumber = false;
                    numbers = "";
                }
            }
        }

        return sum.ToString();
    }

    private bool IsPartNumber(string[] lines, int lineNumber, int characterNumber)
    {
        // top
        if (lineNumber > 0 && IsPartIndicator(lines[lineNumber - 1][characterNumber]))
            return true;
        // left
        if (characterNumber > 0 && IsPartIndicator(lines[lineNumber][characterNumber - 1]))
            return true;
        // right
        if (lines[lineNumber].Length > characterNumber + 1 && IsPartIndicator(lines[lineNumber][characterNumber + 1]))
            return true;
        // bottom
        if (lines.Length > lineNumber + 1 && IsPartIndicator(lines[lineNumber + 1][characterNumber]))
            return true;
        // left-top
        if (characterNumber > 0 && lineNumber > 0 && IsPartIndicator(lines[lineNumber - 1][characterNumber - 1]))
            return true;
        // left-bottom
        if (characterNumber > 0 && lines.Length > lineNumber + 1 && IsPartIndicator(lines[lineNumber + 1][characterNumber - 1]))
            return true;
        // right-top
        if (lines[lineNumber].Length > characterNumber + 1 && lineNumber > 0 && IsPartIndicator(lines[lineNumber - 1][characterNumber + 1]))
            return true;
        // right-bottom
        if (lines[lineNumber].Length > characterNumber + 1 && lines.Length > lineNumber + 1 && IsPartIndicator(lines[lineNumber + 1][characterNumber + 1]))
            return true;
        
        return false;
    }

    private bool IsPartIndicator(char character)
    {
        return !char.IsDigit(character) && character != '.';
    }
}