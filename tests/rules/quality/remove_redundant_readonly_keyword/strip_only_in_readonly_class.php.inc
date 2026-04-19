<?php

final readonly class ReadonlyOne
{
    private readonly string $a;
}

class BareTwo
{
    private readonly string $b;
}

-----
<?php

final readonly class ReadonlyOne
{
    private string $a;
}

class BareTwo
{
    private readonly string $b;
}
