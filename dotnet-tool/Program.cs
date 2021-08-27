﻿using System;
using System.Diagnostics;
using System.Collections.Generic;
using System.ComponentModel;
using System.Reflection;

namespace ruget
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] resNames = Assembly.GetExecutingAssembly().GetManifestResourceNames();
            foreach (string resName in resNames)
                Console.WriteLine(resName);
            // var process = new Process();
            // process.StartInfo.UseShellExecute = false;
            // process.StartInfo.FileName = "cargo";
            // process.StartInfo.ArgumentList.Add("run");
            // process.StartInfo.ArgumentList.Add("--bin");
            // process.StartInfo.ArgumentList.Add("ruget");
            // process.StartInfo.ArgumentList.Add("--");
            // foreach (var arg in args)
            // {
            //     process.StartInfo.ArgumentList.Add(arg);
            // }
            // process.Start();
            // process.WaitForExit();
            // Environment.Exit(process.ExitCode);
        }
    }
}
