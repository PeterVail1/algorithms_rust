One quick way to compare your matching solution is to redirect the output of your console to a file and then diff that file with the corresponding provided file (diff finds and returns all the parts of a file that are different). 

For example:

COMPILE CODE
javac *.java

RUN CODE
java Driver -d inputs/100-500-250.in > 100-500-250.d.out_0     

Here the ">" tells the shell to write the output for your program to 100-500-250.d.out_0 instead of the console ie i/o redirection. 

COMPARE RESULTS
diff 100-500-250.d.out_0 100-500-250.d.out

If the diff command returns nothing, then you know your solution works for that input. One thing to note if you do this in your vscode terminal, you need to make sure to compile your code in the terminal and not assume that it will use the debugger's previous compilation.