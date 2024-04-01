-module(fibonacci).
-export([num/1, sequence/1, max/1, num_lucas/1, sequence_lucas/1, max_lucas/1]).

num(Num) ->
    % Dict = dict:from_list([]),
    % New_Dict = dict:append(1, "a", Dict),
    % io:format("~p~n", [New_Dict]),
    if Num > 0 ->
        {Result,_} = get_fibonacci(Num, []),
        io:format("~p~n", [Result]);
    Num =< 0 ->
        throw("Invalid number.")
    end.

sequence(Count) ->
    if Count > 0 -> display_sequence(Count, 1);
        Count =< 0 -> throw("Invalid number.")
    end.

display_sequence(Count, Num) ->
    % Display the first "Num" fibonacci numbers
    if
        Count > 0 ->
            {Fibonacci,_} = get_fibonacci(Num, []),
            io:format("~p, ", [Fibonacci]),
            display_sequence(Count - 1, Num + 1);
        Count =< 0 ->
            io:format("~n")
    end.

max(Max) ->
    if Max > 0 -> display_max(Max, 1);
        Max =< 0 -> throw("Invalid number.")
    end.

display_max(Max, Num) ->
    % Display the fibonacci numbers up to a maximum number.
    {Fibonacci,_} = get_fibonacci(Num, []),
    if
        Fibonacci =< Max ->
            io:format("~p, ", [Fibonacci]),
            display_max(Max, Num + 1);
        Fibonacci > Max ->
            io:format("~n")
    end.

get_fibonacci(Num, Recorded_Nums) ->
    % This function gets the fibonacci number in the "Num"-th position.
    % TODO: It uses Recorded_Nums to memoize numbers and run faster.

    % io:format("Recorded Nums: ~p", [Recorded_Nums]),
    if
        Num =< 1 -> {0, Recorded_Nums};
        Num == 2 -> {1, Recorded_Nums};
        Num >= 3 ->
                    {New_Num_2, New_Record_2} = get_fibonacci(Num - 2, Recorded_Nums),
                    {New_Num_1, New_Record_1} = get_fibonacci(Num - 1, New_Record_2),
                    {New_Num_1 + New_Num_2, New_Record_1 ++ [New_Num_1 + New_Num_2]}
    end.

num_lucas(Num) ->
    {Result,_} = get_lucas(Num, []),
    io:format("~p~n", [Result]).

sequence_lucas(Count) ->
    if Count > 0 -> display_sequence_lucas(Count, 1);
        Count =< 0 -> throw("Invalid number.")
    end.

display_sequence_lucas(Count, Num) ->
    % Display the first "Num" lucas numbers
    if
        Count > 0 ->
            {Lucas,_} = get_lucas(Num, []),
            io:format("~p, ", [Lucas]),
            display_sequence_lucas(Count - 1, Num + 1);
        Count =< 0 ->
            io:format("~n")
    end.

max_lucas(Max) ->
    if Max > 0 -> display_max_lucas(Max, 1);
        Max =< 0 -> throw("Invalid number.")
    end.

display_max_lucas(Max, Num) ->
    % Display the lucas numbers up to a maximum number.
    {Lucas,_} = get_lucas(Num, []),
    if
        Lucas =< Max ->
            io:format("~p, ", [Lucas]),
            display_max(Max, Num + 1);
        Lucas > Max ->
            io:format("~n")
    end.

get_lucas(Num, Recorded_Nums) ->
    % This function gets the lucas number in the "Num"-th position.
    % TODO: It uses Recorded_Nums to memoize numbers and run faster.

     if
        Num =< 1 -> {2, Recorded_Nums};
        Num == 2 -> {1, Recorded_Nums};
        Num >= 3 ->
                    {New_Num_2, New_Record_2} = get_lucas(Num - 2, Recorded_Nums),
                    {New_Num_1, New_Record_1} = get_lucas(Num - 1, New_Record_2),
                    {New_Num_1 + New_Num_2, New_Record_1 ++ [New_Num_1 + New_Num_2]}
    end.