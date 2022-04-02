module Parsing

type ParserContext =
    { Column: int
      Row: int
      Chars: char list }

type ParserResult<'a, 'b> = Result<'a * ParserContext, 'b>

type Parser<'a, 'b> = ParserContext -> ParserResult<'a, 'b>

let error ctx msg =
    sprintf "[%d:%d] %s" ctx.Row ctx.Column msg

let char c ctx =
    match ctx.Chars with
    | d :: ds when c = d ->
        Ok(
            c,
            { ctx with
                Column = ctx.Column + 1
                Chars = ds }
        )
    | d :: _ ->
        error ctx (sprintf "Expected %c, but found %c instead." c d)
        |> Error
    | [] ->
        error ctx (sprintf "Expected %c, but found EOL instead." c)
        |> Error

let opt p ctx =
    match p ctx with
    | Ok (res, ctx) -> Ok(Some res, ctx)
    | Error _ -> Ok(None, ctx)

let rec many (p: Parser<'a, 'b>) ctx : ParserResult<'a list, 'b> =
    match p ctx with
    | Ok (firstResult, remainingCtx) ->
        match many p remainingCtx with
        | Ok (child, finalCtx) -> Ok(firstResult :: child, finalCtx)
        | Error _ -> Ok([], remainingCtx)
    | Error _ -> Ok([], ctx)

let rec many1 (p: Parser<'a, 'b>) ctx : ParserResult<'a list, 'b> =
    match p ctx with
    | Ok (firstResult, remainingCtx) ->
        match many p remainingCtx with
        | Ok (child, finalCtx) -> Ok(firstResult :: child, finalCtx)
        | _ -> failwithf "many can't fail"
    | Error e -> Error e

let orElse (a: Parser<'a, 'b>) (b: Parser<'a, 'b>) (ctx: ParserContext) : ParserResult<'a, 'b> =
    match a ctx with
    | Ok res -> Ok(res)
    | Error _ -> b ctx

let (<|>) = orElse

let andThen (a: Parser<'a, 'c>) (b: Parser<'b, 'c>) ctx =
    match a ctx with
    | Error e -> Error e
    | Ok (a', ctx) ->
        match b ctx with
        | Error e -> Error e
        | Ok (b', ctx) -> Ok((a', b'), ctx)

let (.>>.) = andThen

let pMap (f: 'a -> 'b) (p: Parser<'a, 'c>) (ctx: ParserContext) : ParserResult<'b, 'c> =
    match p ctx with
    | Ok (res, ctx) -> Ok(f res, ctx)
    | Error e -> Error e

let (|>>) x f = pMap f x

let mustParse (res: ParserResult<'a, 'b>) =
    match res with
    | Error e -> failwithf "%A" e
    | Ok (res, ctx) ->
        match ctx.Chars with
        | [] -> res
        | _ -> failwithf "unparsed chars: %A" ctx
