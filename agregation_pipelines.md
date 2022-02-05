# MongoDB Aggregations

This are some of the pipelines used in this project, you can copy and paste them in the MongoDB Compass to see how they works.

## Projects collection

### get_projects_grouped_by_client

```js
[
  {
    $lookup: {
      from: 'clients',
      localField: 'client',
      foreignField: '_id',
      as: 'client_name',
    },
  },
  {
    $sort: {
      updatedAt: -1,
    },
  },
  {
    $project: {
      _id: '$_id',
      name: '$name',
      color: '$color',
      client_name: {
        $ifNull: [
          {
            $arrayElemAt: ['$client_name.name', 0],
          },
          'No Client',
        ],
      },
      subprojects: '$subprojects',
    },
  },
  {
    $group: {
      _id: '$client_name',
      projects: {
        $push: '$$ROOT',
      },
    },
  },
];
```

## Tasks Collection

### get_tasks_grouped_by_date

```js
[
  {
    $lookup: {
      from: 'projects',
      localField: 'project',
      foreignField: '_id',
      as: 'project',
    },
  },
  {
    $lookup: {
      from: 'clients',
      localField: 'project.client',
      foreignField: '_id',
      as: 'client',
    },
  },
  {
    $project: {
      _id: '$_id',
      name: '$name',
      initial_time: '$initial_time',
      end_time: '$end_time',
      project: {
        $arrayElemAt: ['$project.name', 0],
      },
      project_color: {
        $arrayElemAt: ['$project.color', 0],
      },
      client: {
        $arrayElemAt: ['$client.name', 0],
      },
    },
  },
  {
    $group: {
      _id: {
        $dateToString: {
          format: '%Y-%m-%d',
          date: '$initial_time',
        },
      },
      tasks: {
        $push: '$$ROOT',
      },
      total_time: {
        $sum: {
          $divide: [
            {
              $subtract: ['$end_time', '$initial_time'],
            },
            1000,
          ],
        },
      },
    },
  },
  {
    $facet: {
      details: [
        {
          $count: 'count',
        },
      ],
      dates: [
        {
          $sort: {
            _id: -1,
          },
        },
        {
          $skip: 3,
        },
        {
          $limit: 2,
        },
      ],
    },
  },
];
```
