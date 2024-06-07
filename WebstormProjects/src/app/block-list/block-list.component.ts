import { Component, OnInit } from '@angular/core';
import { BlockService } from '../block.service';
import { Record } from "../record.model";

@Component({
  selector: 'app-block-list',
  templateUrl: './block-list.component.html',
  styleUrls: ['./block-list.component.css']
})

export class BlockListComponent implements OnInit {
  records: Record[] = [];

  constructor(private dataService: BlockService) { }

  ngOnInit(): void {
    this.dataService.getHeights().subscribe(data => {
      this.records = data;
      console.log(this.records);  // Debug: 查看返回的数据
    });
  }
}
