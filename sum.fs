open System

let memoize f =
    let dict = new System.Collections.Generic.Dictionary<_,_>()
    fun n l ->
        match dict.TryGetValue((n, l)) with
        | (true, v) -> v
        | _ ->
            let temp = f n l
            dict.Add((n, l), temp)
            temp

let rec count = memoize(fun (n: int) (l: int) -> 
    // printfn "  compute count(%d, %d)" n l

    if l = 0 then
        if n = 0 then
            1
        else
            0
    else if l = 1 then
        if n = 0 || n = 1 then
            1
        else
            0
    else
        let mutable s = count n (l - 1)
        let mutable v = l
        while n - v >= 0 do
            s <- s + count (n - v) (l - 1)
            v <- v * l
        s
)

[<EntryPoint>]
let main (args: string[]) =
    if args.Length > 0 then
        let n = int args.[0]
        printfn "count(%d) = %d" n (count n n)
    else
        printfn "enter args"
    0