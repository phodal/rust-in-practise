missing-arg-error = 错误: Please provide a number as argument.
input-parse-error = 错误: Could not parse input `{ $input }`. Reason: { $reason }
response-msg =
    { $value ->
        [one] "{ $input }" has one Collatz step.
       *[other] "{ $input }" has { $value } Collatz steps.
    }
