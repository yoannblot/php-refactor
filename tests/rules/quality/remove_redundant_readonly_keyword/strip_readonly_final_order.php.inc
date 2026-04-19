<?php

readonly final class Foo
{
    protected readonly int $count;
}

-----
<?php

readonly final class Foo
{
    protected int $count;
}
