import { Component, OnInit, OnDestroy } from '@angular/core';
import { BlockService } from '../block.service';
import { Subscription } from 'rxjs';
import { ChartType, ChartOptions, ChartDataSets} from 'chart.js';
// import { Label } from 'ng2-charts';

@Component({
  selector: 'app-block-list',
  templateUrl: './block-list.component.html',
  styleUrls: ['./block-list.component.css']
})

export class BlockListComponent implements OnInit, OnDestroy {

  blocks: any[] = [];
  news: any[] = [];
  displayedColumns_block: string[] = ['height_block', 'fee_block', 'hash_block'];
  displayedColumns_news: string[] = ['id_news', 'title_news'];

  public barChartOptions: ChartOptions = {
    responsive: true,
  };
  public barChartLabels: string[] = [];
  public barChartType: ChartType = 'bar';
  public barChartLegend = true;
  public barChartPlugins = [];

  public barChartData: ChartDataSets[] = [
    { data: [], label: 'Fees' }
  ];
  public barChartData2: ChartDataSets[] = [
    { data: [], label: 'N_tx' }
  ];

  private blockIntervalId: any;
  private newsIntervalId: any;
  private blockSubscription: Subscription = new Subscription();
  private newsSubscription: Subscription = new Subscription();

  constructor(private dataService: BlockService) { }

  ngOnInit(): void {

    this.dataService.getBlocks().subscribe(blocks => {
      this.blocks = blocks;
      const fees = blocks.map(block => block.fee);
      this.barChartLabels = blocks.map(block => block.height.toString());
      this.barChartData[0].data = fees;
    });

    this.startPolling();
    this.dataService.getBlocks().subscribe(data => {
      this.blocks = data.slice(0,10);
    });
    this.dataService.getNews().subscribe(response => {
      this.news = response.slice(0,10);
    });

  }

  startPolling(): void {
    this.blockIntervalId = setInterval(() => {
      this.blockSubscription = this.dataService.getBlocks().subscribe(data => {
        this.blocks = data.slice(0, 10);
        console.log('Blocks:', this.blocks);
      });
    }, 5000);  // 每5秒轮询一次blocks

    this.newsIntervalId = setInterval(() => {
      this.newsSubscription = this.dataService.getNews().subscribe(response => {
        this.news = response.slice(0, 10);
        console.log('News:', this.news);
      });
    }, 10000);  // 每10秒轮询一次news
  }

  ngOnDestroy(): void {
    if (this.blockIntervalId) {
      clearInterval(this.blockIntervalId);
    }
    if (this.newsIntervalId) {
      clearInterval(this.newsIntervalId);
    }
    this.blockSubscription.unsubscribe();
    this.newsSubscription.unsubscribe();
  }

}
