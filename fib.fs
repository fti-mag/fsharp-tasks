open System

let memoize f =
    let dict = new System.Collections.Generic.Dictionary<_,_>()
    fun n ->
        match dict.TryGetValue n with
        | (true, v) -> v
        | _ ->
            let temp = f n
            dict.Add (n, temp)
            temp

let rec fib = memoize (fun n ->
    printfn "  compute fib(%d)" n
    if n <= 1 then
        n
    else
        (fib (n-1) + fib (n-2)) % 100000007
)

let rec printFib (args: string[]) = 
    if args.Length <> 0 then
        let n = int args.[0]
        printfn "fib(%d) = %d" n (fib n)
        printFib args.[1..]

[<EntryPoint>]
let main (args: string[]) =
    printFib args
    0