---- MODULE SPS ----
EXTENDS Naturals
VARIABLE events
Init == events = <<>>
Next == events' = Append(events, [tick |-> 0])
====
