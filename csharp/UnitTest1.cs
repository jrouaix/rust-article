using System;
using System.Collections.Generic;
using System.Linq;
using Xunit;

namespace CsharpExamples
{
    public class UnitTests
    {
        [Fact]
        public void FailOnAddWhileEnumerate()
        {
            var list = new List<int>() { 1, 2, 3, 4 };
            for (int i = 5; i < 10; i++)
            {
                list.Add(i);
            }
            foreach (var j in list)
            {
                list.Add(j * j);
            }
        }
    }
}
