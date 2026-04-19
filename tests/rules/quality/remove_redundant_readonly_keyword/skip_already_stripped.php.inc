<?php

final readonly class AlreadyDone
{
    private string $name;

    public function __construct(
        private int $id,
    ) {}
}
