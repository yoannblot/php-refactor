<?php

final readonly class ActivityDashboardFactory
{
    public function __construct(
        private readonly TeamRepository $teamRepository,
        private readonly PeriodFactory $periodFactory,
    ) {}
}

-----
<?php

final readonly class ActivityDashboardFactory
{
    public function __construct(
        private TeamRepository $teamRepository,
        private PeriodFactory $periodFactory,
    ) {}
}
