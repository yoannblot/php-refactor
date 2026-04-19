<?php

final readonly class Mixed
{
    private readonly string $label;

    public function __construct(
        private readonly int $id,
        protected readonly string $name,
    ) {}
}

-----
<?php

final readonly class Mixed
{
    private string $label;

    public function __construct(
        private int $id,
        protected string $name,
    ) {}
}
