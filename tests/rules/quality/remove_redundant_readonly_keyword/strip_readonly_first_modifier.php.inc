<?php

readonly class Foo
{
    public readonly string $bar;
}

-----
<?php

readonly class Foo
{
    public string $bar;
}
